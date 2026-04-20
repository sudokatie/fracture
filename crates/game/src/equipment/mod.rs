//! Equipment systems for Fracture gameplay.
//!
//! Provides phase suits for sickness protection and stability equipment
//! for dimensional stabilization operations.

mod phase_suits;
pub mod stability_equip;

pub use phase_suits::{PhaseSuit, PhaseSuitTier, MAX_DURABILITY};
pub use stability_equip::{AnchorBuilder, StabilityDetector, VoidTether};
