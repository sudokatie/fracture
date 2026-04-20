//! Fracture creature systems.
//!
//! Provides hostile and passive creatures specific to the
//! dimensional fracture gameplay mechanics.

mod hostile;
mod passive;

pub use hostile::{AbilityResult, HostileCreature, HostileType};
pub use passive::{PassiveCreature, PassiveType};
