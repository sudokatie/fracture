//! Dimension rendering systems.
//!
//! Provides visual effects, transitions, and ghost block rendering
//! for the Fracture dimension system.

mod ghost_blocks;
mod transitions;
mod visual_effects;

pub use ghost_blocks::GhostBlockRenderer;
pub use transitions::{FractureVisual, FractureVisuals, PhaseShiftVisuals};
pub use visual_effects::{
    get_color_tint, get_fog_color, get_fog_density, get_light_level, DimensionVisuals,
};
