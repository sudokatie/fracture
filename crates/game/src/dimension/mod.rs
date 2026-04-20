//! Dimension gameplay systems for weak points, anchors, and phase shifting.
//!
//! Provides game-level weak point management, dimensional anchors,
//! and phase shift mechanics that work with the engine's dimension state
//! and fracture systems.

mod anchors;
mod phase_shift;
mod weak_points;

pub use anchors::{AnchorManager, AnchorTier, DimensionalAnchor};
pub use phase_shift::{
    PhaseShiftCost, PhaseShiftManager, PhaseShiftResult, DEFAULT_MAX_COOLDOWN, MAX_SHIFT_SICKNESS,
    MIN_SHIFT_ENERGY,
};
pub use weak_points::{WeakPoint, WeakPointManager};

// Re-export engine types for convenience
pub use engine_physics::dimension::{
    Dimension, DimensionProperties, DimensionState, FractureEngine, FractureEvent, FractureType,
    get_dimension_properties,
};
