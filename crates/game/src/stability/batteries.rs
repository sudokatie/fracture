//! Stability batteries for energy storage.
//!
//! Portable energy storage devices of varying capacities.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Battery capacity tiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StabilityBatteryTier {
    /// Small battery (100 capacity).
    Small,
    /// Medium battery (500 capacity).
    Medium,
    /// Large battery (2000 capacity).
    Large,
}

impl StabilityBatteryTier {
    /// Get the capacity for this tier.
    #[must_use]
    pub fn capacity(&self) -> f32 {
        match self {
            StabilityBatteryTier::Small => 100.0,
            StabilityBatteryTier::Medium => 500.0,
            StabilityBatteryTier::Large => 2000.0,
        }
    }

    /// Get all tier variants.
    #[must_use]
    pub fn all() -> &'static [StabilityBatteryTier] {
        &[
            StabilityBatteryTier::Small,
            StabilityBatteryTier::Medium,
            StabilityBatteryTier::Large,
        ]
    }
}

impl fmt::Display for StabilityBatteryTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StabilityBatteryTier::Small => write!(f, "Small"),
            StabilityBatteryTier::Medium => write!(f, "Medium"),
            StabilityBatteryTier::Large => write!(f, "Large"),
        }
    }
}

/// A stability battery for storing energy.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StabilityBattery {
    /// Battery tier.
    tier: StabilityBatteryTier,
    /// Maximum capacity.
    capacity: f32,
    /// Current charge level.
    charge: f32,
}

impl StabilityBattery {
    /// Create a new fully charged battery of the given tier.
    #[must_use]
    pub fn new(tier: StabilityBatteryTier) -> Self {
        let capacity = tier.capacity();
        Self {
            tier,
            capacity,
            charge: capacity,
        }
    }

    /// Discharge energy from the battery.
    ///
    /// Returns the actual amount discharged (may be less if insufficient).
    pub fn discharge(&mut self, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let discharged = amount.min(self.charge);
        self.charge -= discharged;
        discharged
    }

    /// Recharge the battery.
    ///
    /// Returns the actual amount recharged (capped at capacity).
    pub fn recharge(&mut self, amount: f32) -> f32 {
        if amount <= 0.0 {
            return 0.0;
        }

        let space = self.capacity - self.charge;
        let recharged = amount.min(space);
        self.charge += recharged;
        recharged
    }

    /// Get remaining charge as a percentage (0.0 to 1.0).
    #[must_use]
    pub fn remaining_percentage(&self) -> f32 {
        if self.capacity <= 0.0 {
            return 0.0;
        }
        (self.charge / self.capacity).clamp(0.0, 1.0)
    }

    /// Check if the battery is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.charge <= 0.0
    }

    /// Check if the battery is fully charged.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.charge >= self.capacity
    }

    /// Get the battery tier.
    #[must_use]
    pub fn tier(&self) -> StabilityBatteryTier {
        self.tier
    }

    /// Get the battery capacity.
    #[must_use]
    pub fn capacity(&self) -> f32 {
        self.capacity
    }

    /// Get the current charge level.
    #[must_use]
    pub fn charge(&self) -> f32 {
        self.charge
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_tier_capacity() {
        assert!((StabilityBatteryTier::Small.capacity() - 100.0).abs() < f32::EPSILON);
        assert!((StabilityBatteryTier::Medium.capacity() - 500.0).abs() < f32::EPSILON);
        assert!((StabilityBatteryTier::Large.capacity() - 2000.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_battery_tier_display() {
        assert_eq!(format!("{}", StabilityBatteryTier::Small), "Small");
        assert_eq!(format!("{}", StabilityBatteryTier::Medium), "Medium");
        assert_eq!(format!("{}", StabilityBatteryTier::Large), "Large");
    }

    #[test]
    fn test_battery_new() {
        let battery = StabilityBattery::new(StabilityBatteryTier::Medium);
        assert_eq!(battery.tier(), StabilityBatteryTier::Medium);
        assert!((battery.capacity() - 500.0).abs() < f32::EPSILON);
        assert!((battery.charge() - 500.0).abs() < f32::EPSILON);
        assert!(battery.is_full());
    }

    #[test]
    fn test_battery_discharge() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);

        let discharged = battery.discharge(30.0);
        assert!((discharged - 30.0).abs() < f32::EPSILON);
        assert!((battery.charge() - 70.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_battery_discharge_insufficient() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);
        battery.discharge(80.0);

        let discharged = battery.discharge(50.0);
        assert!((discharged - 20.0).abs() < f32::EPSILON);
        assert!(battery.is_empty());
    }

    #[test]
    fn test_battery_recharge() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);
        battery.discharge(50.0);

        let recharged = battery.recharge(30.0);
        assert!((recharged - 30.0).abs() < f32::EPSILON);
        assert!((battery.charge() - 80.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_battery_recharge_capped() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);
        battery.discharge(20.0);

        let recharged = battery.recharge(50.0);
        assert!((recharged - 20.0).abs() < f32::EPSILON);
        assert!(battery.is_full());
    }

    #[test]
    fn test_battery_remaining_percentage() {
        let mut battery = StabilityBattery::new(StabilityBatteryTier::Small);
        assert!((battery.remaining_percentage() - 1.0).abs() < f32::EPSILON);

        battery.discharge(50.0);
        assert!((battery.remaining_percentage() - 0.5).abs() < f32::EPSILON);
    }
}
