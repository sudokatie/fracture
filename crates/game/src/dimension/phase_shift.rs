//! Phase shift system for transitioning between dimensions.
//!
//! Manages player dimension shifts with energy costs, sickness checks,
//! and cooldown mechanics.

use std::fmt;

use engine_physics::dimension::Dimension;

/// Energy cost tiers for phase shifting.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhaseShiftCost {
    /// Low cost shift (10 energy).
    Low,
    /// Medium cost shift (25 energy).
    Medium,
    /// High cost shift (50 energy).
    High,
}

impl PhaseShiftCost {
    /// Get the energy cost value.
    #[must_use]
    pub fn value(&self) -> f32 {
        match self {
            PhaseShiftCost::Low => 10.0,
            PhaseShiftCost::Medium => 25.0,
            PhaseShiftCost::High => 50.0,
        }
    }
}

impl fmt::Display for PhaseShiftCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhaseShiftCost::Low => write!(f, "Low (10)"),
            PhaseShiftCost::Medium => write!(f, "Medium (25)"),
            PhaseShiftCost::High => write!(f, "High (50)"),
        }
    }
}

/// Result of attempting a phase shift.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhaseShiftResult {
    /// Shift completed successfully.
    Success,
    /// Not enough energy to shift.
    InsufficientEnergy,
    /// Sickness level too high to shift safely.
    SicknessTooHigh,
    /// Still on cooldown from previous shift.
    CooldownActive,
}

impl fmt::Display for PhaseShiftResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhaseShiftResult::Success => write!(f, "Success"),
            PhaseShiftResult::InsufficientEnergy => write!(f, "InsufficientEnergy"),
            PhaseShiftResult::SicknessTooHigh => write!(f, "SicknessTooHigh"),
            PhaseShiftResult::CooldownActive => write!(f, "CooldownActive"),
        }
    }
}

/// Default cooldown duration in seconds.
pub const DEFAULT_MAX_COOLDOWN: f32 = 10.0;

/// Minimum energy required for any shift.
pub const MIN_SHIFT_ENERGY: f32 = 10.0;

/// Maximum sickness level that allows shifting.
pub const MAX_SHIFT_SICKNESS: f32 = 75.0;

/// Manages phase shifting between dimensions.
#[derive(Clone, Debug)]
pub struct PhaseShiftManager {
    /// Current dimension the player is in.
    current_dimension: Dimension,
    /// Remaining cooldown in seconds.
    shift_cooldown: f32,
    /// Maximum cooldown duration.
    max_cooldown: f32,
    /// Total number of shifts performed.
    total_shifts: u32,
}

impl PhaseShiftManager {
    /// Create a new phase shift manager starting in Prime dimension.
    #[must_use]
    pub fn new() -> Self {
        Self {
            current_dimension: Dimension::Prime,
            shift_cooldown: 0.0,
            max_cooldown: DEFAULT_MAX_COOLDOWN,
            total_shifts: 0,
        }
    }

    /// Attempt to shift to a target dimension.
    ///
    /// Checks energy, sickness, and cooldown requirements.
    /// On success, changes dimension, sets cooldown, and increments shift count.
    pub fn shift_to(
        &mut self,
        target: Dimension,
        available_energy: f32,
        current_sickness: f32,
    ) -> PhaseShiftResult {
        // Check cooldown first
        if self.shift_cooldown > 0.0 {
            return PhaseShiftResult::CooldownActive;
        }

        // Check energy
        if available_energy < MIN_SHIFT_ENERGY {
            return PhaseShiftResult::InsufficientEnergy;
        }

        // Check sickness
        if current_sickness >= MAX_SHIFT_SICKNESS {
            return PhaseShiftResult::SicknessTooHigh;
        }

        // Perform the shift
        self.current_dimension = target;
        self.shift_cooldown = self.max_cooldown;
        self.total_shifts += 1;

        PhaseShiftResult::Success
    }

    /// Update cooldown timer.
    pub fn tick(&mut self, dt: f32) {
        if self.shift_cooldown > 0.0 {
            self.shift_cooldown = (self.shift_cooldown - dt).max(0.0);
        }
    }

    /// Check if a shift is currently possible (cooldown only).
    #[must_use]
    pub fn can_shift(&self) -> bool {
        self.shift_cooldown <= 0.0
    }

    /// Get the current dimension.
    #[must_use]
    pub fn current_dimension(&self) -> Dimension {
        self.current_dimension
    }

    /// Get remaining cooldown in seconds.
    #[must_use]
    pub fn cooldown_remaining(&self) -> f32 {
        self.shift_cooldown
    }

    /// Get total number of shifts performed.
    #[must_use]
    pub fn shift_count(&self) -> u32 {
        self.total_shifts
    }

    /// Reset to initial state (Prime dimension, zero cooldown).
    pub fn reset(&mut self) {
        self.current_dimension = Dimension::Prime;
        self.shift_cooldown = 0.0;
        self.total_shifts = 0;
    }

    // HUD Display helpers

    /// Get the display color for the current dimension.
    ///
    /// Returns RGB color values (0.0 to 1.0).
    #[must_use]
    pub fn display_color(&self) -> [f32; 3] {
        match self.current_dimension {
            Dimension::Prime => [0.3, 0.7, 1.0],     // Light blue
            Dimension::Inverted => [1.0, 0.5, 0.2],  // Orange
            Dimension::Void => [0.4, 0.4, 0.5],      // Gray
            Dimension::Nexus => [0.7, 0.3, 0.9],     // Purple
        }
    }

    /// Get the display label for the current dimension.
    #[must_use]
    pub fn display_label(&self) -> &'static str {
        match self.current_dimension {
            Dimension::Prime => "PRIME",
            Dimension::Inverted => "INVERTED",
            Dimension::Void => "VOID",
            Dimension::Nexus => "NEXUS",
        }
    }

    /// Get the cooldown progress (0.0 = full cooldown, 1.0 = ready).
    #[must_use]
    pub fn cooldown_progress(&self) -> f32 {
        if self.max_cooldown <= 0.0 {
            return 1.0;
        }
        1.0 - (self.shift_cooldown / self.max_cooldown).clamp(0.0, 1.0)
    }

    /// Get formatted cooldown text.
    #[must_use]
    pub fn cooldown_text(&self) -> Option<String> {
        if self.shift_cooldown <= 0.0 {
            None
        } else {
            Some(format!("{:.1}s", self.shift_cooldown))
        }
    }

    /// Check if shift ability should show as unavailable in HUD.
    #[must_use]
    pub fn is_shift_blocked(&self) -> bool {
        self.shift_cooldown > 0.0
    }

    /// Get the icon name for the current dimension.
    #[must_use]
    pub fn icon(&self) -> &'static str {
        match self.current_dimension {
            Dimension::Prime => "icon_dimension_prime",
            Dimension::Inverted => "icon_dimension_inverted",
            Dimension::Void => "icon_dimension_void",
            Dimension::Nexus => "icon_dimension_nexus",
        }
    }
}

impl Default for PhaseShiftManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_shift_cost_values() {
        assert!((PhaseShiftCost::Low.value() - 10.0).abs() < f32::EPSILON);
        assert!((PhaseShiftCost::Medium.value() - 25.0).abs() < f32::EPSILON);
        assert!((PhaseShiftCost::High.value() - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_phase_shift_cost_display() {
        assert_eq!(format!("{}", PhaseShiftCost::Low), "Low (10)");
        assert_eq!(format!("{}", PhaseShiftCost::Medium), "Medium (25)");
        assert_eq!(format!("{}", PhaseShiftCost::High), "High (50)");
    }

    #[test]
    fn test_phase_shift_result_display() {
        assert_eq!(format!("{}", PhaseShiftResult::Success), "Success");
        assert_eq!(
            format!("{}", PhaseShiftResult::InsufficientEnergy),
            "InsufficientEnergy"
        );
        assert_eq!(
            format!("{}", PhaseShiftResult::SicknessTooHigh),
            "SicknessTooHigh"
        );
        assert_eq!(
            format!("{}", PhaseShiftResult::CooldownActive),
            "CooldownActive"
        );
    }

    #[test]
    fn test_manager_new() {
        let manager = PhaseShiftManager::new();
        assert_eq!(manager.current_dimension(), Dimension::Prime);
        assert!((manager.cooldown_remaining() - 0.0).abs() < f32::EPSILON);
        assert_eq!(manager.shift_count(), 0);
        assert!(manager.can_shift());
    }

    #[test]
    fn test_shift_success() {
        let mut manager = PhaseShiftManager::new();

        let result = manager.shift_to(Dimension::Void, 100.0, 0.0);
        assert_eq!(result, PhaseShiftResult::Success);
        assert_eq!(manager.current_dimension(), Dimension::Void);
        assert_eq!(manager.shift_count(), 1);
        assert!(!manager.can_shift()); // Cooldown active
    }

    #[test]
    fn test_shift_insufficient_energy() {
        let mut manager = PhaseShiftManager::new();

        let result = manager.shift_to(Dimension::Void, 5.0, 0.0);
        assert_eq!(result, PhaseShiftResult::InsufficientEnergy);
        assert_eq!(manager.current_dimension(), Dimension::Prime); // Unchanged
        assert_eq!(manager.shift_count(), 0);
    }

    #[test]
    fn test_shift_sickness_too_high() {
        let mut manager = PhaseShiftManager::new();

        let result = manager.shift_to(Dimension::Void, 100.0, 80.0);
        assert_eq!(result, PhaseShiftResult::SicknessTooHigh);
        assert_eq!(manager.current_dimension(), Dimension::Prime);
    }

    #[test]
    fn test_shift_cooldown_active() {
        let mut manager = PhaseShiftManager::new();

        // First shift succeeds
        manager.shift_to(Dimension::Void, 100.0, 0.0);

        // Second shift fails due to cooldown
        let result = manager.shift_to(Dimension::Nexus, 100.0, 0.0);
        assert_eq!(result, PhaseShiftResult::CooldownActive);
        assert_eq!(manager.current_dimension(), Dimension::Void); // Still in Void
    }

    #[test]
    fn test_tick_reduces_cooldown() {
        let mut manager = PhaseShiftManager::new();
        manager.shift_to(Dimension::Void, 100.0, 0.0);

        assert!(!manager.can_shift());
        manager.tick(5.0);
        assert!(!manager.can_shift());
        assert!((manager.cooldown_remaining() - 5.0).abs() < f32::EPSILON);

        manager.tick(6.0);
        assert!(manager.can_shift());
        assert!((manager.cooldown_remaining() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_reset() {
        let mut manager = PhaseShiftManager::new();
        manager.shift_to(Dimension::Void, 100.0, 0.0);
        manager.shift_to(Dimension::Nexus, 100.0, 0.0); // Will fail but increment won't happen

        manager.reset();
        assert_eq!(manager.current_dimension(), Dimension::Prime);
        assert!((manager.cooldown_remaining() - 0.0).abs() < f32::EPSILON);
        assert_eq!(manager.shift_count(), 0);
        assert!(manager.can_shift());
    }
}
