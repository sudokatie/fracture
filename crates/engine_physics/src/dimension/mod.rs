//! Dimension system for reality-breaking mechanics.
//!
//! Provides the core dimension state tracking, fracture mechanics,
//! and dimension properties for the Fracture game.

mod fractures;
mod state;

pub use fractures::{FractureEngine, FractureEvent, FractureType};
pub use state::{
    get_dimension_properties, Dimension, DimensionProperties, DimensionState,
};
