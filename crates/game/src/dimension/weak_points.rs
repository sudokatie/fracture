//! Weak point tracking and management.
//!
//! Weak points are locations in the world where the fabric of reality
//! is thin and vulnerable to fracturing.

use engine_physics::dimension::Dimension;
use glam::IVec3;
use serde::{Deserialize, Serialize};

/// A weak point in reality that may collapse into a fracture.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeakPoint {
    /// World position of the weak point.
    position: IVec3,
    /// Instability level (0.0 = stable, 1.0 = collapsed).
    instability: f32,
    /// Reinforcement level (0.0 = none, 1.0 = max).
    reinforcement: f32,
    /// The dimension this weak point originated in.
    origin_dimension: Dimension,
    /// Whether this weak point has collapsed.
    collapsed: bool,
}

impl WeakPoint {
    /// Create a new weak point at the given position.
    #[must_use]
    pub fn new(pos: IVec3) -> Self {
        Self {
            position: pos,
            instability: 0.0,
            reinforcement: 0.0,
            origin_dimension: Dimension::Prime,
            collapsed: false,
        }
    }

    /// Create a weak point in a specific dimension.
    #[must_use]
    pub fn in_dimension(pos: IVec3, dimension: Dimension) -> Self {
        Self {
            position: pos,
            instability: 0.0,
            reinforcement: 0.0,
            origin_dimension: dimension,
            collapsed: false,
        }
    }

    /// Tick the weak point, advancing its state.
    ///
    /// Instability increases over time while reinforcement decreases.
    /// Returns false if the weak point has collapsed (instability >= 1.0).
    pub fn tick(&mut self, dt: f32) -> bool {
        if self.collapsed {
            return false;
        }

        // Instability grows faster when not reinforced
        let growth_rate = 0.01 * (1.0 - self.reinforcement * 0.8);
        self.instability = (self.instability + growth_rate * dt).min(1.0);

        // Reinforcement decays over time
        self.reinforcement = (self.reinforcement - 0.02 * dt).max(0.0);

        // Check for collapse
        if self.instability >= 1.0 {
            self.collapsed = true;
            return false;
        }

        true
    }

    /// Add reinforcement to the weak point.
    pub fn reinforce(&mut self, strength: f32) {
        self.reinforcement = (self.reinforcement + strength).min(1.0);
        // Reinforcing also slightly reduces instability
        self.instability = (self.instability - strength * 0.1).max(0.0);
    }

    /// Get the current instability level.
    #[must_use]
    pub fn instability_level(&self) -> f32 {
        self.instability
    }

    /// Get the current reinforcement level.
    #[must_use]
    pub fn reinforcement_level(&self) -> f32 {
        self.reinforcement
    }

    /// Check if the weak point has collapsed.
    #[must_use]
    pub fn is_collapsed(&self) -> bool {
        self.collapsed
    }

    /// Get the position of the weak point.
    #[must_use]
    pub fn position(&self) -> IVec3 {
        self.position
    }

    /// Get the origin dimension.
    #[must_use]
    pub fn origin_dimension(&self) -> Dimension {
        self.origin_dimension
    }
}

/// Manager for tracking multiple weak points in the world.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WeakPointManager {
    /// All tracked weak points.
    weak_points: Vec<WeakPoint>,
}

impl WeakPointManager {
    /// Create a new empty weak point manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            weak_points: Vec::new(),
        }
    }

    /// Add a weak point, returning its index.
    pub fn add_weak_point(&mut self, wp: WeakPoint) -> usize {
        let index = self.weak_points.len();
        self.weak_points.push(wp);
        index
    }

    /// Tick all weak points, returning indices of collapsed ones.
    pub fn tick_all(&mut self, dt: f32) -> Vec<usize> {
        let mut collapsed = Vec::new();

        for (index, wp) in self.weak_points.iter_mut().enumerate() {
            if !wp.is_collapsed() && !wp.tick(dt) {
                collapsed.push(index);
            }
        }

        collapsed
    }

    /// Reinforce a weak point by index.
    ///
    /// Returns false if the index is invalid or the weak point is collapsed.
    pub fn reinforce(&mut self, index: usize, strength: f32) -> bool {
        if let Some(wp) = self.weak_points.get_mut(index) {
            if !wp.is_collapsed() {
                wp.reinforce(strength);
                return true;
            }
        }
        false
    }

    /// Find the nearest weak point to a position.
    #[must_use]
    pub fn nearest_weak_point(&self, pos: IVec3) -> Option<&WeakPoint> {
        self.weak_points
            .iter()
            .filter(|wp| !wp.is_collapsed())
            .min_by_key(|wp| {
                let diff = wp.position() - pos;
                diff.x * diff.x + diff.y * diff.y + diff.z * diff.z
            })
    }

    /// Get the total number of weak points (including collapsed).
    #[must_use]
    pub fn weak_point_count(&self) -> usize {
        self.weak_points.len()
    }

    /// Get the number of active (non-collapsed) weak points.
    #[must_use]
    pub fn active_weak_points(&self) -> usize {
        self.weak_points.iter().filter(|wp| !wp.is_collapsed()).count()
    }

    /// Get a weak point by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&WeakPoint> {
        self.weak_points.get(index)
    }

    /// Get a mutable weak point by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut WeakPoint> {
        self.weak_points.get_mut(index)
    }

    /// Iterate over all weak points.
    pub fn iter(&self) -> impl Iterator<Item = &WeakPoint> {
        self.weak_points.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weak_point_new() {
        let pos = IVec3::new(10, 20, 30);
        let wp = WeakPoint::new(pos);

        assert_eq!(wp.position(), pos);
        assert!((wp.instability_level() - 0.0).abs() < f32::EPSILON);
        assert!((wp.reinforcement_level() - 0.0).abs() < f32::EPSILON);
        assert!(!wp.is_collapsed());
        assert_eq!(wp.origin_dimension(), Dimension::Prime);
    }

    #[test]
    fn test_weak_point_in_dimension() {
        let wp = WeakPoint::in_dimension(IVec3::ZERO, Dimension::Void);
        assert_eq!(wp.origin_dimension(), Dimension::Void);
    }

    #[test]
    fn test_weak_point_tick_increases_instability() {
        let mut wp = WeakPoint::new(IVec3::ZERO);
        let initial = wp.instability_level();

        wp.tick(1.0);
        assert!(wp.instability_level() > initial);
    }

    #[test]
    fn test_weak_point_tick_decreases_reinforcement() {
        let mut wp = WeakPoint::new(IVec3::ZERO);
        wp.reinforce(0.5);
        let initial = wp.reinforcement_level();

        wp.tick(1.0);
        assert!(wp.reinforcement_level() < initial);
    }

    #[test]
    fn test_weak_point_collapse() {
        let mut wp = WeakPoint::new(IVec3::ZERO);

        // Tick until collapsed
        for _ in 0..200 {
            if !wp.tick(1.0) {
                break;
            }
        }

        assert!(wp.is_collapsed());
        assert!((wp.instability_level() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_weak_point_reinforce() {
        let mut wp = WeakPoint::new(IVec3::ZERO);
        wp.reinforce(0.5);

        assert!((wp.reinforcement_level() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_weak_point_reinforce_caps_at_one() {
        let mut wp = WeakPoint::new(IVec3::ZERO);
        wp.reinforce(0.8);
        wp.reinforce(0.5);

        assert!((wp.reinforcement_level() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_weak_point_reinforce_reduces_instability() {
        let mut wp = WeakPoint::new(IVec3::ZERO);
        wp.tick(5.0); // Build up some instability
        let before = wp.instability_level();

        wp.reinforce(0.5);
        assert!(wp.instability_level() < before);
    }

    #[test]
    fn test_manager_new() {
        let manager = WeakPointManager::new();
        assert_eq!(manager.weak_point_count(), 0);
        assert_eq!(manager.active_weak_points(), 0);
    }

    #[test]
    fn test_manager_add_weak_point() {
        let mut manager = WeakPointManager::new();
        let index = manager.add_weak_point(WeakPoint::new(IVec3::ZERO));

        assert_eq!(index, 0);
        assert_eq!(manager.weak_point_count(), 1);
    }

    #[test]
    fn test_manager_tick_all() {
        let mut manager = WeakPointManager::new();
        manager.add_weak_point(WeakPoint::new(IVec3::new(0, 0, 0)));
        manager.add_weak_point(WeakPoint::new(IVec3::new(1, 0, 0)));

        let collapsed = manager.tick_all(1.0);
        assert!(collapsed.is_empty()); // Not enough time to collapse
    }

    #[test]
    fn test_manager_reinforce() {
        let mut manager = WeakPointManager::new();
        let index = manager.add_weak_point(WeakPoint::new(IVec3::ZERO));

        assert!(manager.reinforce(index, 0.5));
        assert!((manager.get(index).unwrap().reinforcement_level() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_manager_reinforce_invalid_index() {
        let mut manager = WeakPointManager::new();
        assert!(!manager.reinforce(999, 0.5));
    }

    #[test]
    fn test_manager_nearest_weak_point() {
        let mut manager = WeakPointManager::new();
        manager.add_weak_point(WeakPoint::new(IVec3::new(0, 0, 0)));
        manager.add_weak_point(WeakPoint::new(IVec3::new(10, 0, 0)));
        manager.add_weak_point(WeakPoint::new(IVec3::new(100, 0, 0)));

        let nearest = manager.nearest_weak_point(IVec3::new(8, 0, 0));
        assert!(nearest.is_some());
        assert_eq!(nearest.unwrap().position(), IVec3::new(10, 0, 0));
    }

    #[test]
    fn test_manager_active_vs_total() {
        let mut manager = WeakPointManager::new();
        manager.add_weak_point(WeakPoint::new(IVec3::new(0, 0, 0)));
        manager.add_weak_point(WeakPoint::new(IVec3::new(1, 0, 0)));

        assert_eq!(manager.weak_point_count(), 2);
        assert_eq!(manager.active_weak_points(), 2);

        // Collapse one by ticking a lot
        for _ in 0..200 {
            manager.tick_all(1.0);
        }

        // Total unchanged, active reduced
        assert_eq!(manager.weak_point_count(), 2);
        assert!(manager.active_weak_points() < 2);
    }
}
