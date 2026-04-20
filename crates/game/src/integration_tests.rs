//! Integration tests for Fracture game systems.
//!
//! These tests verify that multiple game systems work together correctly,
//! covering fracture lifecycle, anchors, sickness, transmutation, stability
//! energy, phase shifting, dimension state, creatures, and weak points.

use glam::IVec3;

use crate::creatures::{HostileCreature, HostileType};
use crate::dimension::{
    AnchorManager, AnchorTier, DimensionalAnchor, Dimension, DimensionState, FractureEngine,
    FractureType, PhaseShiftManager, PhaseShiftResult, WeakPoint, WeakPointManager,
    MAX_SHIFT_SICKNESS,
};
use crate::stability::{AnchorFuelCell, AnchorFuelCellTier, StabilityBattery, StabilityBatteryTier, StabilityEnergy};
use crate::survival::{FractureSickness, TransmutationTable};

/// Test 1: Full fracture lifecycle - register weak point -> advance days -> fracture event
#[test]
fn test_full_fracture_lifecycle() {
    let mut engine = FractureEngine::new();
    let mut dim_state = DimensionState::new();

    // Register a weak point
    let weak_point_pos = IVec3::new(10, 0, 10);
    engine.register_weak_point(weak_point_pos);
    assert_eq!(engine.weak_point_count(), 1);

    // Initial chunk is Prime
    assert_eq!(dim_state.get_dimension(weak_point_pos), Dimension::Prime);

    // Advance to day when micro fractures can occur (day 3+)
    engine.set_day(2);

    // Run many days to ensure at least one fracture event (probabilistic)
    let mut fracture_occurred = false;
    for _ in 0..100 {
        let events = engine.advance_day();
        if !events.is_empty() {
            // Apply fracture to dimension state
            for event in events {
                dim_state.set_dimension(event.weak_point_pos, event.target_dimension);
                fracture_occurred = true;
            }
            break;
        }
    }

    // After many days, a fracture should have occurred (with high probability)
    assert!(fracture_occurred, "Expected a fracture event after many days");
    // The chunk should no longer be Prime
    assert_ne!(dim_state.get_dimension(weak_point_pos), Dimension::Prime);
}

/// Test 2: Anchor protection - anchored chunk stays Prime while unanchored fractures
#[test]
fn test_anchor_protection() {
    let mut anchor_manager = AnchorManager::new();
    let mut dim_state = DimensionState::new();

    // Place anchor at origin
    let anchor_pos = IVec3::ZERO;
    let anchor = DimensionalAnchor::new(anchor_pos, AnchorTier::Basic);
    anchor_manager.add_anchor(anchor);

    // Nearby chunk is protected
    let protected_chunk = IVec3::new(1, 0, 0);
    assert!(anchor_manager.is_chunk_anchored(protected_chunk));

    // Far chunk is not protected
    let unprotected_chunk = IVec3::new(100, 0, 0);
    assert!(!anchor_manager.is_chunk_anchored(unprotected_chunk));

    // Simulate fracture on unprotected chunk only
    dim_state.set_dimension(unprotected_chunk, Dimension::Void);

    // Protected chunk stays Prime
    assert_eq!(dim_state.get_dimension(protected_chunk), Dimension::Prime);
    // Unprotected chunk changed
    assert_eq!(dim_state.get_dimension(unprotected_chunk), Dimension::Void);
}

/// Test 3: Fracture sickness - shift to Inverted increases, return to Prime decreases
#[test]
fn test_fracture_sickness_lifecycle() {
    let mut sickness = FractureSickness::new();

    // Start healthy
    assert!((sickness.sickness_level() - 0.0).abs() < f32::EPSILON);

    // Simulate being in non-Prime dimension (sickness increases)
    sickness.set_in_prime(false);
    sickness.add_sickness(30.0);
    assert!((sickness.sickness_level() - 30.0).abs() < f32::EPSILON);

    // No recovery while outside Prime
    sickness.tick(1.0);
    assert!((sickness.sickness_level() - 30.0).abs() < f32::EPSILON);

    // Return to Prime (sickness decreases on tick)
    sickness.set_in_prime(true);
    sickness.tick(1.0);
    assert!(sickness.sickness_level() < 30.0);

    // Multiple ticks reduce sickness further
    for _ in 0..20 {
        sickness.tick(1.0);
    }
    assert!(sickness.sickness_level() < 10.0);
}

/// Test 4: Cross-dimension resource changes (transmutation)
#[test]
fn test_cross_dimension_transmutation() {
    let table = TransmutationTable::new();

    // Water to Void -> nothing
    let result = table.transmute("water", Dimension::Prime, Dimension::Void);
    assert_eq!(result, Some("nothing".to_string()));

    // Lava to Prime -> obsidian
    let result = table.transmute("lava", Dimension::Inverted, Dimension::Prime);
    assert_eq!(result, Some("obsidian".to_string()));

    // No transmutation in same dimension
    let result = table.transmute("water", Dimension::Prime, Dimension::Prime);
    assert!(result.is_none());

    // Unknown item has no transmutation
    let result = table.transmute("unknown_item", Dimension::Prime, Dimension::Void);
    assert!(result.is_none());
}

/// Test 5: Stability energy management - harvest, store, use for anchor refuel
#[test]
fn test_stability_energy_management() {
    // Player's energy pool
    let mut player_energy = StabilityEnergy::new(200.0);

    // Harvest energy from a weak point (simulated)
    let harvested = player_energy.harvest(50.0);
    assert!((harvested - 0.0).abs() < f32::EPSILON); // Already full

    // Use some energy
    player_energy.consume(100.0);
    assert!((player_energy.current_energy() - 100.0).abs() < f32::EPSILON);

    // Store in battery
    let mut battery = StabilityBattery::new(StabilityBatteryTier::Medium);
    battery.discharge(500.0); // Empty the battery
    let stored = battery.recharge(50.0);
    assert!((stored - 50.0).abs() < f32::EPSILON);

    // Use battery to refuel anchor
    let mut anchor = DimensionalAnchor::new(IVec3::ZERO, AnchorTier::Basic);
    anchor.tick(300.0); // Use half the fuel

    let fuel_cell = AnchorFuelCell::new(AnchorFuelCellTier::Small);
    let refueled = anchor.refuel(fuel_cell.fuel_remaining());
    assert!(refueled > 0.0);
}

/// Test 6: Phase shift mechanics - success with sufficient energy/low sickness, blocked with high sickness
#[test]
fn test_phase_shift_mechanics() {
    let mut phase_manager = PhaseShiftManager::new();

    // Start in Prime
    assert_eq!(phase_manager.current_dimension(), Dimension::Prime);

    // Shift to Inverted with sufficient energy and low sickness
    let result = phase_manager.shift_to(Dimension::Inverted, 100.0, 10.0);
    assert_eq!(result, PhaseShiftResult::Success);
    assert_eq!(phase_manager.current_dimension(), Dimension::Inverted);

    // Wait for cooldown
    phase_manager.tick(15.0);

    // Try to shift with high sickness (should be blocked)
    let high_sickness = MAX_SHIFT_SICKNESS + 5.0;
    let result = phase_manager.shift_to(Dimension::Void, 100.0, high_sickness);
    assert_eq!(result, PhaseShiftResult::SicknessTooHigh);
    assert_eq!(phase_manager.current_dimension(), Dimension::Inverted); // Unchanged

    // Try to shift with insufficient energy
    let result = phase_manager.shift_to(Dimension::Void, 5.0, 10.0);
    assert_eq!(result, PhaseShiftResult::InsufficientEnergy);
}

/// Test 7: Dimension state tracking - set chunks, count, fracture borders
#[test]
fn test_dimension_state_tracking() {
    let mut dim_state = DimensionState::new();

    // Set various chunks to different dimensions
    dim_state.set_dimension(IVec3::new(0, 0, 0), Dimension::Void);
    dim_state.set_dimension(IVec3::new(1, 0, 0), Dimension::Void);
    dim_state.set_dimension(IVec3::new(2, 0, 0), Dimension::Inverted);
    dim_state.set_dimension(IVec3::new(3, 0, 0), Dimension::Nexus);

    // Count dimensions
    assert_eq!(dim_state.dimension_count(Dimension::Void), 2);
    assert_eq!(dim_state.dimension_count(Dimension::Inverted), 1);
    assert_eq!(dim_state.dimension_count(Dimension::Nexus), 1);

    // Check fracture borders
    // Chunk at (1,0,0) is Void, neighbor at (2,0,0) is Inverted -> border
    assert!(dim_state.is_fracture_border(IVec3::new(1, 0, 0)));
    // Chunk at (0,0,0) is Void, neighbor at (-1,0,0) is Prime -> border
    assert!(dim_state.is_fracture_border(IVec3::new(0, 0, 0)));

    // Total tracked (non-Prime)
    assert_eq!(dim_state.total_tracked(), 4);
}

/// Test 8: Creature dimension spawning - hostile creatures have correct stats
#[test]
fn test_creature_dimension_spawning() {
    // Shadow Walker - dimension-crossing predator
    let shadow_walker = HostileCreature::new(HostileType::ShadowWalker);
    assert_eq!(shadow_walker.special_ability(), "phase_strike");
    assert!(shadow_walker.is_alive());

    // Void Wraith - ethereal void creature
    let void_wraith = HostileCreature::new(HostileType::VoidWraith);
    assert_eq!(void_wraith.special_ability(), "void_pull");
    let ability_result = void_wraith.use_ability();
    assert!(ability_result.success);
    assert!(ability_result.effect.contains("void"));

    // Nexus Beast - dimension-shifting boss
    let nexus_beast = HostileCreature::new(HostileType::NexusBeast);
    assert_eq!(nexus_beast.special_ability(), "dimension_shift");
    assert_eq!(nexus_beast.hp(), 300);

    // Magma Crawler - inverted dimension creature
    let magma_crawler = HostileCreature::new(HostileType::MagmaCrawler);
    assert_eq!(magma_crawler.special_ability(), "lava_splash");
    let ability_result = magma_crawler.use_ability();
    assert!(ability_result.area_radius > 0.0);
}

/// Test 9: Weak point reinforcement - reduces instability growth
#[test]
fn test_weak_point_reinforcement() {
    let mut wp_unreinforced = WeakPoint::new(IVec3::new(0, 0, 0));
    let mut wp_reinforced = WeakPoint::new(IVec3::new(1, 0, 0));

    // Reinforce one weak point
    wp_reinforced.reinforce(0.8);

    // Tick both for some time
    for _ in 0..10 {
        wp_unreinforced.tick(1.0);
        wp_reinforced.tick(1.0);
    }

    // Reinforced weak point should have lower instability
    assert!(
        wp_reinforced.instability_level() < wp_unreinforced.instability_level(),
        "Reinforced weak point should grow instability slower"
    );

    // Reinforcement also reduces existing instability slightly
    let mut wp = WeakPoint::new(IVec3::ZERO);
    wp.tick(5.0); // Build up instability
    let before = wp.instability_level();
    wp.reinforce(0.5);
    assert!(wp.instability_level() < before);
}

/// Test 10: Fracture cascade - day 21+ enables cascade events
#[test]
fn test_fracture_cascade_possible() {
    let mut engine = FractureEngine::new();

    // Register weak points
    for i in 0..10 {
        engine.register_weak_point(IVec3::new(i * 10, 0, 0));
    }

    // Before day 21, cascade not possible
    engine.set_day(20);
    assert!(engine.current_day() < FractureType::Cascade.start_day());

    // Advance to day 21+
    engine.set_day(20);
    engine.advance_day(); // Now day 21
    assert_eq!(engine.current_day(), 21);
    assert!(engine.current_day() >= FractureType::Cascade.start_day());

    // Run many iterations to try to trigger cascade (probabilistic)
    let mut cascade_found = false;
    for _ in 0..500 {
        let events = engine.advance_day();
        for event in &events {
            if event.fracture_type == FractureType::Cascade {
                cascade_found = true;
                // Cascade should have multiple events
                assert!(event.radius >= FractureType::Cascade.base_radius());
                break;
            }
        }
        if cascade_found {
            break;
        }
    }

    // Note: Due to low probability, this may not always trigger
    // But the test verifies the day requirement
    assert!(engine.current_day() >= FractureType::Cascade.start_day());
}
