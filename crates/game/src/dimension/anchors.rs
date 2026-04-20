//! Dimensional anchors for stabilizing reality.
//!
//! Anchors prevent fractures from forming within their radius
//! by consuming fuel to maintain dimensional stability.

use std::fmt;

use glam::IVec3;
use serde::{Deserialize, Serialize};

/// Tiers of dimensional anchors with increasing power.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnchorTier {
    /// Basic anchor - small radius, short duration.
    Basic,
    /// Standard anchor - medium radius and duration.
    Standard,
    /// Military-grade anchor - large radius, long duration.
    Military,
}

impl fmt::Display for AnchorTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnchorTier::Basic => write!(f, "Basic"),
            AnchorTier::Standard => write!(f, "Standard"),
            AnchorTier::Military => write!(f, "Military"),
        }
    }
}

impl AnchorTier {
    /// Get the protection radius in chunks.
    #[must_use]
    pub fn radius(&self) -> u32 {
        match self {
            AnchorTier::Basic => 3,
            AnchorTier::Standard => 5,
            AnchorTier::Military => 7,
        }
    }

    /// Get the maximum fuel capacity in seconds.
    #[must_use]
    pub fn max_fuel(&self) -> f32 {
        match self {
            AnchorTier::Basic => 600.0,       // 10 minutes
            AnchorTier::Standard => 1800.0,   // 30 minutes
            AnchorTier::Military => 3600.0,   // 60 minutes
        }
    }

    /// Get all tier variants.
    #[must_use]
    pub fn all() -> &'static [AnchorTier] {
        &[AnchorTier::Basic, AnchorTier::Standard, AnchorTier::Military]
    }
}

/// A dimensional anchor that stabilizes a region of chunks.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DimensionalAnchor {
    /// World position of the anchor.
    position: IVec3,
    /// Tier of the anchor.
    tier: AnchorTier,
    /// Protection radius in chunks.
    radius: u32,
    /// Remaining fuel in seconds.
    fuel_remaining: f32,
    /// Whether the anchor is active.
    active: bool,
}

impl DimensionalAnchor {
    /// Create a new anchor at the given position.
    #[must_use]
    pub fn new(pos: IVec3, tier: AnchorTier) -> Self {
        Self {
            position: pos,
            tier,
            radius: tier.radius(),
            fuel_remaining: tier.max_fuel(),
            active: true,
        }
    }

    /// Tick the anchor, consuming fuel.
    ///
    /// Returns false if fuel is depleted.
    pub fn tick(&mut self, dt: f32) -> bool {
        if !self.active {
            return false;
        }

        self.fuel_remaining -= dt;
        if self.fuel_remaining <= 0.0 {
            self.fuel_remaining = 0.0;
            self.active = false;
            return false;
        }

        true
    }

    /// Add fuel to the anchor.
    ///
    /// Returns the actual amount added (capped at max capacity).
    pub fn refuel(&mut self, amount: f32) -> f32 {
        let max = self.tier.max_fuel();
        let space = max - self.fuel_remaining;
        let added = amount.min(space);
        self.fuel_remaining += added;

        // Reactivate if fuel was added to a depleted anchor
        if self.fuel_remaining > 0.0 {
            self.active = true;
        }

        added
    }

    /// Check if a chunk position is within the anchor's protection radius.
    #[must_use]
    pub fn covers_chunk(&self, chunk_pos: IVec3) -> bool {
        if !self.active {
            return false;
        }

        let diff = chunk_pos - self.position;
        let distance_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z;
        let radius = self.radius as i32;
        distance_sq <= radius * radius
    }

    /// Deactivate the anchor and return remaining fuel.
    pub fn deactivate(&mut self) -> f32 {
        self.active = false;
        self.fuel_remaining
    }

    /// Get remaining fuel as a percentage (0.0 to 1.0).
    #[must_use]
    pub fn remaining_percentage(&self) -> f32 {
        let max = self.tier.max_fuel();
        if max <= 0.0 {
            return 0.0;
        }
        (self.fuel_remaining / max).clamp(0.0, 1.0)
    }

    /// Get the position of the anchor.
    #[must_use]
    pub fn position(&self) -> IVec3 {
        self.position
    }

    /// Get the tier of the anchor.
    #[must_use]
    pub fn tier(&self) -> AnchorTier {
        self.tier
    }

    /// Get the protection radius.
    #[must_use]
    pub fn radius(&self) -> u32 {
        self.radius
    }

    /// Get remaining fuel in seconds.
    #[must_use]
    pub fn fuel_remaining(&self) -> f32 {
        self.fuel_remaining
    }

    /// Check if the anchor is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }
}

/// Manager for tracking multiple anchors in the world.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AnchorManager {
    /// All tracked anchors.
    anchors: Vec<DimensionalAnchor>,
}

impl AnchorManager {
    /// Create a new empty anchor manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            anchors: Vec::new(),
        }
    }

    /// Add an anchor, returning its index.
    pub fn add_anchor(&mut self, anchor: DimensionalAnchor) -> usize {
        let index = self.anchors.len();
        self.anchors.push(anchor);
        index
    }

    /// Remove an anchor by index.
    pub fn remove_anchor(&mut self, index: usize) -> Option<DimensionalAnchor> {
        if index < self.anchors.len() {
            Some(self.anchors.remove(index))
        } else {
            None
        }
    }

    /// Check if a chunk is protected by any active anchor.
    #[must_use]
    pub fn is_chunk_anchored(&self, chunk_pos: IVec3) -> bool {
        self.anchors.iter().any(|a| a.covers_chunk(chunk_pos))
    }

    /// Tick all anchors, returning indices of depleted ones.
    pub fn tick_all(&mut self, dt: f32) -> Vec<usize> {
        let mut depleted = Vec::new();

        for (index, anchor) in self.anchors.iter_mut().enumerate() {
            if anchor.is_active() && !anchor.tick(dt) {
                depleted.push(index);
            }
        }

        depleted
    }

    /// Get the total number of anchors.
    #[must_use]
    pub fn anchor_count(&self) -> usize {
        self.anchors.len()
    }

    /// Get the number of active anchors.
    #[must_use]
    pub fn active_anchors(&self) -> usize {
        self.anchors.iter().filter(|a| a.is_active()).count()
    }

    /// Get an anchor by index.
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&DimensionalAnchor> {
        self.anchors.get(index)
    }

    /// Get a mutable anchor by index.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut DimensionalAnchor> {
        self.anchors.get_mut(index)
    }

    /// Iterate over all anchors.
    pub fn iter(&self) -> impl Iterator<Item = &DimensionalAnchor> {
        self.anchors.iter()
    }

    /// Find all anchors covering a chunk position.
    pub fn anchors_at(&self, chunk_pos: IVec3) -> Vec<&DimensionalAnchor> {
        self.anchors
            .iter()
            .filter(|a| a.covers_chunk(chunk_pos))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anchor_tier_display() {
        assert_eq!(format!("{}", AnchorTier::Basic), "Basic");
        assert_eq!(format!("{}", AnchorTier::Standard), "Standard");
        assert_eq!(format!("{}", AnchorTier::Military), "Military");
    }

    #[test]
    fn test_anchor_tier_radius() {
        assert_eq!(AnchorTier::Basic.radius(), 3);
        assert_eq!(AnchorTier::Standard.radius(), 5);
        assert_eq!(AnchorTier::Military.radius(), 7);
    }

    #[test]
    fn test_anchor_tier_max_fuel() {
        assert!((AnchorTier::Basic.max_fuel() - 600.0).abs() < f32::EPSILON);
        assert!((AnchorTier::Standard.max_fuel() - 1800.0).abs() < f32::EPSILON);
        assert!((AnchorTier::Military.max_fuel() - 3600.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_anchor_new() {
        let anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Standard);

        assert_eq!(anchor.position(), IVec3::ZERO);
        assert_eq!(anchor.tier(), AnchorTier::Standard);
        assert_eq!(anchor.radius(), 5);
        assert!((anchor.fuel_remaining() - 1800.0).abs() < f32::EPSILON);
        assert!(anchor.is_active());
    }

    #[test]
    fn test_anchor_tick_consumes_fuel() {
        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);
        let initial = anchor.fuel_remaining();

        anchor.tick(10.0);
        assert!(anchor.fuel_remaining() < initial);
        assert!(anchor.is_active());
    }

    #[test]
    fn test_anchor_tick_depletes() {
        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);

        // Tick past max fuel
        let result = anchor.tick(700.0);
        assert!(!result);
        assert!(!anchor.is_active());
        assert!((anchor.fuel_remaining() - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_anchor_refuel() {
        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);
        anchor.tick(100.0); // Use some fuel

        let added = anchor.refuel(50.0);
        assert!((added - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_anchor_refuel_caps_at_max() {
        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);
        anchor.tick(10.0); // Use some fuel (now at 590)

        let added = anchor.refuel(100.0); // Try to add more than space
        assert!((added - 10.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_anchor_refuel_reactivates() {
        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);
        anchor.tick(700.0); // Deplete
        assert!(!anchor.is_active());

        anchor.refuel(50.0);
        assert!(anchor.is_active());
    }

    #[test]
    fn test_anchor_covers_chunk() {
        let anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic); // radius 3

        assert!(anchor.covers_chunk(IVec3::new(0, 0, 0)));
        assert!(anchor.covers_chunk(IVec3::new(1, 1, 1))); // distance sqrt(3) < 3
        assert!(anchor.covers_chunk(IVec3::new(2, 0, 0)));
        assert!(!anchor.covers_chunk(IVec3::new(5, 0, 0))); // distance 5 > 3
    }

    #[test]
    fn test_anchor_covers_chunk_inactive() {
        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);
        anchor.deactivate();

        assert!(!anchor.covers_chunk(IVec3::ZERO)); // Inactive doesn't cover
    }

    #[test]
    fn test_anchor_deactivate() {
        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);
        anchor.tick(100.0); // Use some fuel

        let remaining = anchor.deactivate();
        assert!((remaining - 500.0).abs() < f32::EPSILON);
        assert!(!anchor.is_active());
    }

    #[test]
    fn test_anchor_remaining_percentage() {
        let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);
        assert!((anchor.remaining_percentage() - 1.0).abs() < f32::EPSILON);

        anchor.tick(300.0); // Half fuel
        assert!((anchor.remaining_percentage() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_manager_new() {
        let manager = AnchorManager::new();
        assert_eq!(manager.anchor_count(), 0);
        assert_eq!(manager.active_anchors(), 0);
    }

    #[test]
    fn test_manager_add_anchor() {
        let mut manager = AnchorManager::new();
        let index = manager.add_anchor(DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic));

        assert_eq!(index, 0);
        assert_eq!(manager.anchor_count(), 1);
    }

    #[test]
    fn test_manager_remove_anchor() {
        let mut manager = AnchorManager::new();
        manager.add_anchor(DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic));

        let removed = manager.remove_anchor(0);
        assert!(removed.is_some());
        assert_eq!(manager.anchor_count(), 0);
    }

    #[test]
    fn test_manager_is_chunk_anchored() {
        let mut manager = AnchorManager::new();
        manager.add_anchor(DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic));

        assert!(manager.is_chunk_anchored(IVec3::new(1, 0, 0)));
        assert!(!manager.is_chunk_anchored(IVec3::new(10, 0, 0)));
    }

    #[test]
    fn test_manager_tick_all() {
        let mut manager = AnchorManager::new();
        manager.add_anchor(DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic));

        let depleted = manager.tick_all(10.0);
        assert!(depleted.is_empty());

        // Deplete it
        let depleted = manager.tick_all(600.0);
        assert_eq!(depleted.len(), 1);
    }

    #[test]
    fn test_manager_active_vs_total() {
        let mut manager = AnchorManager::new();
        manager.add_anchor(DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic));
        manager.add_anchor(DimensionalAnchor::new(IVec3::new(10, 0, 0), AnchorTier::Basic));

        assert_eq!(manager.anchor_count(), 2);
        assert_eq!(manager.active_anchors(), 2);

        manager.tick_all(700.0); // Deplete both
        assert_eq!(manager.anchor_count(), 2);
        assert_eq!(manager.active_anchors(), 0);
    }
}
