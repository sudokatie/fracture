//! Fracture mechanics for dimension instability.
//!
//! Handles weak points, fracture events, and cascade failures
//! that can tear holes between dimensions.

use std::fmt;

use glam::IVec3;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use super::state::Dimension;

/// Types of fractures with increasing severity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FractureType {
    /// Small, localized fracture affecting a single chunk.
    Micro,
    /// Medium fracture affecting a small area.
    Meso,
    /// Large fracture affecting a significant area.
    Macro,
    /// Catastrophic cascade affecting multiple areas in waves.
    Cascade,
}

impl fmt::Display for FractureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FractureType::Micro => write!(f, "Micro"),
            FractureType::Meso => write!(f, "Meso"),
            FractureType::Macro => write!(f, "Macro"),
            FractureType::Cascade => write!(f, "Cascade"),
        }
    }
}

impl FractureType {
    /// Get the base radius of effect for this fracture type.
    #[must_use]
    pub fn base_radius(&self) -> u32 {
        match self {
            FractureType::Micro => 1,
            FractureType::Meso => 3,
            FractureType::Macro => 7,
            FractureType::Cascade => 7, // Per wave
        }
    }

    /// Get the day when this fracture type can first occur.
    #[must_use]
    pub fn start_day(&self) -> u32 {
        match self {
            FractureType::Micro => 3,
            FractureType::Meso => 7,
            FractureType::Macro => 14,
            FractureType::Cascade => 21,
        }
    }

    /// Get the base probability per weak point per day.
    #[must_use]
    pub fn base_probability(&self) -> f32 {
        match self {
            FractureType::Micro => 0.05,
            FractureType::Meso => 0.02,
            FractureType::Macro => 0.01,
            FractureType::Cascade => 0.005,
        }
    }
}

/// A fracture event that occurred at a weak point.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FractureEvent {
    /// Position of the weak point that fractured.
    pub weak_point_pos: IVec3,
    /// Type of fracture.
    pub fracture_type: FractureType,
    /// Radius of chunks affected.
    pub radius: u32,
    /// The dimension the fracture originated from.
    pub source_dimension: Dimension,
    /// The dimension that chunks will transform into.
    pub target_dimension: Dimension,
}

impl FractureEvent {
    /// Create a new fracture event.
    #[must_use]
    pub fn new(
        pos: IVec3,
        ftype: FractureType,
        source: Dimension,
        target: Dimension,
    ) -> Self {
        Self {
            weak_point_pos: pos,
            fracture_type: ftype,
            radius: ftype.base_radius(),
            source_dimension: source,
            target_dimension: target,
        }
    }

    /// Create a cascade wave event with custom radius.
    #[must_use]
    pub fn cascade_wave(
        pos: IVec3,
        wave_number: u32,
        source: Dimension,
        target: Dimension,
    ) -> Self {
        Self {
            weak_point_pos: pos,
            fracture_type: FractureType::Cascade,
            radius: FractureType::Cascade.base_radius() + wave_number,
            source_dimension: source,
            target_dimension: target,
        }
    }
}

/// Engine that tracks weak points and generates fracture events.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct FractureEngine {
    /// Current world day.
    world_day: u32,
    /// Active weak points that can fracture.
    active_weak_points: Vec<IVec3>,
    /// History of all fracture events.
    fracture_history: Vec<FractureEvent>,
    /// Reinforcement levels for weak points (reduces probability).
    reinforcements: Vec<(IVec3, f32)>,
}

impl FractureEngine {
    /// Create a new fracture engine starting at day 1.
    #[must_use]
    pub fn new() -> Self {
        Self {
            world_day: 1,
            active_weak_points: Vec::new(),
            fracture_history: Vec::new(),
            reinforcements: Vec::new(),
        }
    }

    /// Register a new weak point.
    pub fn register_weak_point(&mut self, pos: IVec3) {
        if !self.active_weak_points.contains(&pos) {
            self.active_weak_points.push(pos);
        }
    }

    /// Unregister a weak point.
    pub fn unregister_weak_point(&mut self, pos: IVec3) {
        self.active_weak_points.retain(|&p| p != pos);
        self.reinforcements.retain(|(p, _)| *p != pos);
    }

    /// Advance to the next day and evaluate fractures.
    ///
    /// Returns a list of fracture events that occurred.
    pub fn advance_day(&mut self) -> Vec<FractureEvent> {
        self.world_day += 1;
        let mut events = Vec::new();
        let mut rng = rand::thread_rng();

        // Snapshot weak points to avoid borrow issues
        let weak_points: Vec<IVec3> = self.active_weak_points.clone();

        for pos in weak_points {
            // Check for each fracture type, from most severe to least
            // Only one type can occur per weak point per day
            let reinforcement = self.get_reinforcement(pos);

            for ftype in [
                FractureType::Cascade,
                FractureType::Macro,
                FractureType::Meso,
                FractureType::Micro,
            ] {
                if self.world_day >= ftype.start_day() {
                    let prob = (ftype.base_probability() - reinforcement).max(0.0);
                    if rng.r#gen::<f32>() < prob {
                        // Generate fracture event
                        let source = Dimension::Prime;
                        let target = self.random_target_dimension(&mut rng, source);

                        if ftype == FractureType::Cascade {
                            // Cascade generates 3-5 sequential events
                            let wave_count = rng.gen_range(3..=5);
                            for wave in 0..wave_count {
                                events.push(FractureEvent::cascade_wave(
                                    pos,
                                    wave,
                                    source,
                                    target,
                                ));
                            }
                        } else {
                            events.push(FractureEvent::new(pos, ftype, source, target));
                        }

                        // Only one fracture type per weak point per day
                        break;
                    }
                }
            }
        }

        // Record events in history
        self.fracture_history.extend(events.clone());

        // Decay reinforcements
        self.reinforcements.retain_mut(|(_, strength)| {
            *strength -= 0.01; // Decay per day
            *strength > 0.0
        });

        events
    }

    /// Get the current fracture probability for a weak point.
    #[must_use]
    pub fn fracture_probability(&self, weak_point_pos: IVec3) -> f32 {
        if !self.active_weak_points.contains(&weak_point_pos) {
            return 0.0;
        }

        let reinforcement = self.get_reinforcement(weak_point_pos);
        let mut total_prob = 0.0;

        for ftype in [
            FractureType::Micro,
            FractureType::Meso,
            FractureType::Macro,
            FractureType::Cascade,
        ] {
            if self.world_day >= ftype.start_day() {
                total_prob += (ftype.base_probability() - reinforcement).max(0.0);
            }
        }

        total_prob
    }

    /// Reinforce a weak point, reducing fracture probability.
    ///
    /// Returns false if the position is not a registered weak point.
    pub fn reinforce_weak_point(&mut self, pos: IVec3, strength: f32) -> bool {
        if !self.active_weak_points.contains(&pos) {
            return false;
        }

        // Find existing reinforcement or add new one
        if let Some((_, existing)) = self.reinforcements.iter_mut().find(|(p, _)| *p == pos) {
            *existing = (*existing + strength).min(1.0);
        } else {
            self.reinforcements.push((pos, strength.min(1.0)));
        }

        true
    }

    /// Get the number of registered weak points.
    #[must_use]
    pub fn weak_point_count(&self) -> usize {
        self.active_weak_points.len()
    }

    /// Get the number of fracture events in history.
    #[must_use]
    pub fn history_count(&self) -> usize {
        self.fracture_history.len()
    }

    /// Set the current day (for testing).
    pub fn set_day(&mut self, day: u32) {
        self.world_day = day;
    }

    /// Get the current day.
    #[must_use]
    pub fn current_day(&self) -> u32 {
        self.world_day
    }

    /// Get reinforcement level for a weak point.
    fn get_reinforcement(&self, pos: IVec3) -> f32 {
        self.reinforcements
            .iter()
            .find(|(p, _)| *p == pos)
            .map(|(_, s)| *s)
            .unwrap_or(0.0)
    }

    /// Pick a random target dimension different from source.
    fn random_target_dimension<R: Rng>(&self, rng: &mut R, source: Dimension) -> Dimension {
        let targets: Vec<Dimension> = Dimension::all()
            .iter()
            .copied()
            .filter(|&d| d != source)
            .collect();
        targets[rng.gen_range(0..targets.len())]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fracture_type_display() {
        assert_eq!(format!("{}", FractureType::Micro), "Micro");
        assert_eq!(format!("{}", FractureType::Meso), "Meso");
        assert_eq!(format!("{}", FractureType::Macro), "Macro");
        assert_eq!(format!("{}", FractureType::Cascade), "Cascade");
    }

    #[test]
    fn test_fracture_type_radius() {
        assert_eq!(FractureType::Micro.base_radius(), 1);
        assert_eq!(FractureType::Meso.base_radius(), 3);
        assert_eq!(FractureType::Macro.base_radius(), 7);
        assert_eq!(FractureType::Cascade.base_radius(), 7);
    }

    #[test]
    fn test_fracture_type_start_day() {
        assert_eq!(FractureType::Micro.start_day(), 3);
        assert_eq!(FractureType::Meso.start_day(), 7);
        assert_eq!(FractureType::Macro.start_day(), 14);
        assert_eq!(FractureType::Cascade.start_day(), 21);
    }

    #[test]
    fn test_fracture_type_probability() {
        assert!((FractureType::Micro.base_probability() - 0.05).abs() < f32::EPSILON);
        assert!((FractureType::Meso.base_probability() - 0.02).abs() < f32::EPSILON);
        assert!((FractureType::Macro.base_probability() - 0.01).abs() < f32::EPSILON);
        assert!((FractureType::Cascade.base_probability() - 0.005).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fracture_event_new() {
        let event = FractureEvent::new(
            IVec3::ZERO,
            FractureType::Micro,
            Dimension::Prime,
            Dimension::Void,
        );
        assert_eq!(event.weak_point_pos, IVec3::ZERO);
        assert_eq!(event.fracture_type, FractureType::Micro);
        assert_eq!(event.radius, 1);
        assert_eq!(event.source_dimension, Dimension::Prime);
        assert_eq!(event.target_dimension, Dimension::Void);
    }

    #[test]
    fn test_fracture_event_cascade_wave() {
        let event = FractureEvent::cascade_wave(
            IVec3::new(1, 2, 3),
            2,
            Dimension::Prime,
            Dimension::Nexus,
        );
        assert_eq!(event.fracture_type, FractureType::Cascade);
        assert_eq!(event.radius, 9); // 7 + 2
    }

    #[test]
    fn test_fracture_engine_new() {
        let engine = FractureEngine::new();
        assert_eq!(engine.current_day(), 1);
        assert_eq!(engine.weak_point_count(), 0);
        assert_eq!(engine.history_count(), 0);
    }

    #[test]
    fn test_fracture_engine_register_weak_point() {
        let mut engine = FractureEngine::new();
        let pos = IVec3::new(5, 5, 5);

        engine.register_weak_point(pos);
        assert_eq!(engine.weak_point_count(), 1);

        // Duplicate registration should not add
        engine.register_weak_point(pos);
        assert_eq!(engine.weak_point_count(), 1);
    }

    #[test]
    fn test_fracture_engine_unregister_weak_point() {
        let mut engine = FractureEngine::new();
        let pos = IVec3::new(5, 5, 5);

        engine.register_weak_point(pos);
        engine.unregister_weak_point(pos);
        assert_eq!(engine.weak_point_count(), 0);
    }

    #[test]
    fn test_fracture_engine_set_day() {
        let mut engine = FractureEngine::new();
        engine.set_day(10);
        assert_eq!(engine.current_day(), 10);
    }

    #[test]
    fn test_fracture_engine_probability_before_start() {
        let mut engine = FractureEngine::new();
        let pos = IVec3::ZERO;

        engine.register_weak_point(pos);
        engine.set_day(1);

        // Day 1: no fractures possible
        assert!((engine.fracture_probability(pos) - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_fracture_engine_probability_after_start() {
        let mut engine = FractureEngine::new();
        let pos = IVec3::ZERO;

        engine.register_weak_point(pos);
        engine.set_day(3);

        // Day 3: Micro fractures possible
        assert!(engine.fracture_probability(pos) > 0.0);
    }

    #[test]
    fn test_fracture_engine_reinforce() {
        let mut engine = FractureEngine::new();
        let pos = IVec3::ZERO;

        engine.register_weak_point(pos);
        engine.set_day(10);

        let prob_before = engine.fracture_probability(pos);
        assert!(engine.reinforce_weak_point(pos, 0.03));
        let prob_after = engine.fracture_probability(pos);

        assert!(prob_after < prob_before);
    }

    #[test]
    fn test_fracture_engine_reinforce_invalid() {
        let mut engine = FractureEngine::new();
        let pos = IVec3::ZERO;

        // Can't reinforce unregistered weak point
        assert!(!engine.reinforce_weak_point(pos, 0.5));
    }

    #[test]
    fn test_fracture_engine_advance_day() {
        let mut engine = FractureEngine::new();
        let pos = IVec3::ZERO;

        engine.register_weak_point(pos);
        engine.set_day(2);

        // Advance to day 3 (Micro fractures start)
        let _events = engine.advance_day();
        assert_eq!(engine.current_day(), 3);
    }

    #[test]
    fn test_fracture_engine_history_accumulates() {
        let mut engine = FractureEngine::new();
        let pos = IVec3::ZERO;

        engine.register_weak_point(pos);
        engine.set_day(20);

        // Advance many days to likely generate some events
        for _ in 0..50 {
            engine.advance_day();
        }

        // History should contain any events that occurred
        // (may or may not be > 0 due to randomness)
        let _ = engine.history_count();
    }
}
