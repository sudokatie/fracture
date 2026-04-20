//! Dimension gameplay systems for weak points and anchors.
//!
//! Provides game-level weak point management and dimensional anchors
//! that work with the engine's dimension state and fracture systems.

mod anchors;
mod weak_points;

pub use anchors::{AnchorManager, AnchorTier, DimensionalAnchor};
pub use weak_points::{WeakPoint, WeakPointManager};

// Re-export engine types for convenience
pub use engine_physics::dimension::{
    Dimension, DimensionProperties, DimensionState, FractureEngine, FractureEvent, FractureType,
    get_dimension_properties,
};
