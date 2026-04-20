//! Fracture survival game client.
//!
//! Core game logic including ECS components, systems, and entity management.
//! Reality-breaking survival mechanics with dimensional phase shifting.

pub mod ai;
pub mod audio;
pub mod building;
pub mod crafting;
pub mod creatures;
pub mod dimension;
pub mod ecs;
pub mod entities;
pub mod equipment;
pub mod inventory;
pub mod networking;
pub mod stability;
pub mod survival;
pub mod world;

#[cfg(test)]
mod integration_tests;
