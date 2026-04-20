//! Audio systems for Fracture gameplay.
//!
//! Provides dimension-specific ambient sounds and fracture/creature audio.

mod dimension_ambient;
mod fracture_audio;

pub use dimension_ambient::{get_ambient_sound, volume_modifier, DimensionAmbient};
pub use fracture_audio::{get_creature_sound, get_fracture_sound, FractureAudio};
