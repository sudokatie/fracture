//! Passive creature types for the Fracture game.
//!
//! Passive creatures do not attack and provide resources
//! or special effects related to dimensional fractures.

use serde::{Deserialize, Serialize};

/// Types of passive creatures in Fracture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PassiveType {
    /// Moth that glows near fractures, found in all dimensions.
    PhaseMoth,
    /// Snail with insulating shell, native to Inverted dimension.
    CrystalSnail,
    /// Fish found on Void platforms, cures dimension sickness.
    VoidFish,
    /// Butterfly exclusive to Nexus, leaves stability trail.
    NexusButterfly,
    /// Hare from Prime that vanishes when reality fractures.
    EchoHare,
}

impl PassiveType {
    /// Get base HP for this creature type.
    #[must_use]
    pub fn base_hp(&self) -> u32 {
        match self {
            PassiveType::PhaseMoth => 8,
            PassiveType::CrystalSnail => 20,
            PassiveType::VoidFish => 12,
            PassiveType::NexusButterfly => 10,
            PassiveType::EchoHare => 15,
        }
    }

    /// Get the drop item for this creature type.
    #[must_use]
    pub fn drop_item(&self) -> &'static str {
        match self {
            PassiveType::PhaseMoth => "stability_dust",
            PassiveType::CrystalSnail => "crystal_shell",
            PassiveType::VoidFish => "void_fillet",
            PassiveType::NexusButterfly => "nexus_dust",
            PassiveType::EchoHare => "echo_fur",
        }
    }

    /// Get the special trait for this creature type.
    #[must_use]
    pub fn special_trait(&self) -> &'static str {
        match self {
            PassiveType::PhaseMoth => "glows_near_fractures",
            PassiveType::CrystalSnail => "insulating_shell",
            PassiveType::VoidFish => "cures_sickness",
            PassiveType::NexusButterfly => "stability_trail",
            PassiveType::EchoHare => "vanishes_when_fractured",
        }
    }

    /// Get display name for this creature type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            PassiveType::PhaseMoth => "Phase Moth",
            PassiveType::CrystalSnail => "Crystal Snail",
            PassiveType::VoidFish => "Void Fish",
            PassiveType::NexusButterfly => "Nexus Butterfly",
            PassiveType::EchoHare => "Echo Hare",
        }
    }

    /// Get the native dimension for this creature.
    #[must_use]
    pub fn native_dimension(&self) -> &'static str {
        match self {
            PassiveType::PhaseMoth => "all",
            PassiveType::CrystalSnail => "inverted",
            PassiveType::VoidFish => "void",
            PassiveType::NexusButterfly => "nexus",
            PassiveType::EchoHare => "prime",
        }
    }
}

/// A passive creature instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassiveCreature {
    /// Type of this creature.
    creature_type: PassiveType,
    /// Current HP.
    hp: u32,
    /// Maximum HP.
    max_hp: u32,
    /// Item dropped when caught/killed.
    drop_item: String,
    /// Special trait identifier.
    special_trait: String,
}

impl PassiveCreature {
    /// Create a new passive creature of the given type.
    #[must_use]
    pub fn new(creature_type: PassiveType) -> Self {
        let max_hp = creature_type.base_hp();
        Self {
            creature_type,
            hp: max_hp,
            max_hp,
            drop_item: creature_type.drop_item().to_string(),
            special_trait: creature_type.special_trait().to_string(),
        }
    }

    /// Get the creature type.
    #[must_use]
    pub fn creature_type(&self) -> PassiveType {
        self.creature_type
    }

    /// Get current HP.
    #[must_use]
    pub fn hp(&self) -> u32 {
        self.hp
    }

    /// Get maximum HP.
    #[must_use]
    pub fn max_hp(&self) -> u32 {
        self.max_hp
    }

    /// Get the drop item name.
    #[must_use]
    pub fn drop_item(&self) -> &str {
        &self.drop_item
    }

    /// Get the special trait name.
    #[must_use]
    pub fn special_trait(&self) -> &str {
        &self.special_trait
    }

    /// Check if the creature is alive.
    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Apply damage to the creature.
    ///
    /// Returns the actual damage dealt.
    pub fn take_damage(&mut self, amount: u32) -> u32 {
        let actual = amount.min(self.hp);
        self.hp = self.hp.saturating_sub(amount);
        actual
    }

    /// Attempt to catch the creature.
    ///
    /// Returns the drop item if successful (creature dies), None otherwise.
    pub fn on_catch(&mut self) -> Option<String> {
        if self.is_alive() {
            self.hp = 0;
            Some(self.drop_item.clone())
        } else {
            None
        }
    }

    /// Check if this creature has a specific trait.
    #[must_use]
    pub fn has_trait(&self, trait_name: &str) -> bool {
        self.special_trait == trait_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passive_type_stats() {
        assert_eq!(PassiveType::PhaseMoth.base_hp(), 8);
        assert_eq!(PassiveType::CrystalSnail.base_hp(), 20);
        assert_eq!(PassiveType::EchoHare.base_hp(), 15);
    }

    #[test]
    fn test_passive_type_drops() {
        assert_eq!(PassiveType::PhaseMoth.drop_item(), "stability_dust");
        assert_eq!(PassiveType::VoidFish.drop_item(), "void_fillet");
        assert_eq!(PassiveType::EchoHare.drop_item(), "echo_fur");
    }

    #[test]
    fn test_passive_type_traits() {
        assert_eq!(PassiveType::PhaseMoth.special_trait(), "glows_near_fractures");
        assert_eq!(PassiveType::VoidFish.special_trait(), "cures_sickness");
        assert_eq!(PassiveType::NexusButterfly.special_trait(), "stability_trail");
    }

    #[test]
    fn test_passive_type_dimensions() {
        assert_eq!(PassiveType::PhaseMoth.native_dimension(), "all");
        assert_eq!(PassiveType::CrystalSnail.native_dimension(), "inverted");
        assert_eq!(PassiveType::VoidFish.native_dimension(), "void");
        assert_eq!(PassiveType::NexusButterfly.native_dimension(), "nexus");
        assert_eq!(PassiveType::EchoHare.native_dimension(), "prime");
    }

    #[test]
    fn test_passive_creature_new() {
        let creature = PassiveCreature::new(PassiveType::CrystalSnail);

        assert_eq!(creature.creature_type(), PassiveType::CrystalSnail);
        assert_eq!(creature.hp(), 20);
        assert_eq!(creature.max_hp(), 20);
        assert_eq!(creature.drop_item(), "crystal_shell");
        assert_eq!(creature.special_trait(), "insulating_shell");
        assert!(creature.is_alive());
    }

    #[test]
    fn test_passive_creature_take_damage() {
        let mut creature = PassiveCreature::new(PassiveType::EchoHare);
        assert_eq!(creature.hp(), 15);

        let dealt = creature.take_damage(5);
        assert_eq!(dealt, 5);
        assert_eq!(creature.hp(), 10);
        assert!(creature.is_alive());
    }

    #[test]
    fn test_passive_creature_death() {
        let mut creature = PassiveCreature::new(PassiveType::PhaseMoth);
        creature.take_damage(10);

        assert_eq!(creature.hp(), 0);
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_passive_creature_overkill() {
        let mut creature = PassiveCreature::new(PassiveType::PhaseMoth);
        let dealt = creature.take_damage(100);

        assert_eq!(dealt, 8); // Only actual HP
        assert_eq!(creature.hp(), 0);
    }

    #[test]
    fn test_passive_creature_on_catch() {
        let mut creature = PassiveCreature::new(PassiveType::VoidFish);
        assert!(creature.is_alive());

        let drop = creature.on_catch();
        assert!(drop.is_some());
        assert_eq!(drop.unwrap(), "void_fillet");
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_passive_creature_on_catch_when_dead() {
        let mut creature = PassiveCreature::new(PassiveType::VoidFish);
        creature.take_damage(100);

        let drop = creature.on_catch();
        assert!(drop.is_none());
    }

    #[test]
    fn test_passive_creature_has_trait() {
        let creature = PassiveCreature::new(PassiveType::NexusButterfly);

        assert!(creature.has_trait("stability_trail"));
        assert!(!creature.has_trait("glows_near_fractures"));
    }

    #[test]
    fn test_all_passive_types_have_unique_drops() {
        let types = [
            PassiveType::PhaseMoth,
            PassiveType::CrystalSnail,
            PassiveType::VoidFish,
            PassiveType::NexusButterfly,
            PassiveType::EchoHare,
        ];

        let drops: Vec<_> = types.iter().map(|t| t.drop_item()).collect();
        for (i, drop) in drops.iter().enumerate() {
            for (j, other) in drops.iter().enumerate() {
                if i != j {
                    assert_ne!(drop, other, "Duplicate drop items found");
                }
            }
        }
    }

    #[test]
    fn test_display_names() {
        assert_eq!(PassiveType::PhaseMoth.display_name(), "Phase Moth");
        assert_eq!(PassiveType::CrystalSnail.display_name(), "Crystal Snail");
        assert_eq!(PassiveType::VoidFish.display_name(), "Void Fish");
        assert_eq!(PassiveType::NexusButterfly.display_name(), "Nexus Butterfly");
        assert_eq!(PassiveType::EchoHare.display_name(), "Echo Hare");
    }
}
