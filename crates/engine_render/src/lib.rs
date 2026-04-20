//! Rendering system for the Fracture game engine.
//!
//! Provides GPU abstraction, voxel rendering, and visual effects
//! including dimension-specific rendering for the reality-breaking survival game.

pub mod backend;
pub mod camera;
pub mod dimension;
pub mod fog;
pub mod ghost_block;
pub mod lighting;
mod renderer;
pub mod sky;
pub mod voxel;

pub use renderer::{TriangleRenderer, Vertex};
