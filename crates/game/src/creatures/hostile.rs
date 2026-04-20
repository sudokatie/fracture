//! Hostile creature types for the Fracture game.
//!
//! Hostile creatures attack players and have special abilities
//! related to dimensional fractures.

use serde::{Deserialize, Serialize};

/// Types of hostile creatures in Fracture.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HostileType {
    /// Phase-shifting predator that attacks from other dimensions.
    ShadowWalker,
    /// Slow but powerful volcanic creature with area damage.
    MagmaCrawler,
    /// Fast serpent that explodes into shrapnel on death.
    CrystalSerpent,
    /// Ethereal creature that pulls victims toward the void.
    VoidWraith,
    /// Massive beast that shifts attack patterns between dimensions.
    NexusBeast,
}

impl HostileType {
    /// Get base HP for this creature type.
    #[must_use]
    pub fn base_hp(&self) -> u32 {
        match self {
            HostileType::ShadowWalker => 100,
            HostileType::MagmaCrawler => 150,
            HostileType::CrystalSerpent => 120,
            HostileType::VoidWraith => 80,
            HostileType::NexusBeast => 300,
        }
    }

    /// Get base damage for this creature type.
    #[must_use]
    pub fn base_damage(&self) -> u32 {
        match self {
            HostileType::ShadowWalker => 18,
            HostileType::MagmaCrawler => 22,
            HostileType::CrystalSerpent => 15,
            HostileType::VoidWraith => 12,
            HostileType::NexusBeast => 30,
        }
    }

    /// Get base movement speed for this creature type.
    #[must_use]
    pub fn base_speed(&self) -> f32 {
        match self {
            HostileType::ShadowWalker => 1.5,
            HostileType::MagmaCrawler => 0.8,
            HostileType::CrystalSerpent => 2.0,
            HostileType::VoidWraith => 1.8,
            HostileType::NexusBeast => 1.0,
        }
    }

    /// Get the special ability name for this creature type.
    #[must_use]
    pub fn special_ability(&self) -> &'static str {
        match self {
            HostileType::ShadowWalker => "phase_strike",
            HostileType::MagmaCrawler => "lava_splash",
            HostileType::CrystalSerpent => "crystal_shatter",
            HostileType::VoidWraith => "void_pull",
            HostileType::NexusBeast => "dimension_shift",
        }
    }

    /// Get display name for this creature type.
    #[must_use]
    pub fn display_name(&self) -> &'static str {
        match self {
            HostileType::ShadowWalker => "Shadow Walker",
            HostileType::MagmaCrawler => "Magma Crawler",
            HostileType::CrystalSerpent => "Crystal Serpent",
            HostileType::VoidWraith => "Void Wraith",
            HostileType::NexusBeast => "Nexus Beast",
        }
    }
}

/// Result of using a special ability.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AbilityResult {
    /// Whether the ability was successfully used.
    pub success: bool,
    /// Damage dealt by the ability.
    pub damage: u32,
    /// Area of effect radius (0 for single target).
    pub area_radius: f32,
    /// Description of the effect.
    pub effect: String,
}

impl AbilityResult {
    /// Create a new ability result.
    #[must_use]
    pub fn new(success: bool, damage: u32, area_radius: f32, effect: String) -> Self {
        Self {
            success,
            damage,
            area_radius,
            effect,
        }
    }

    /// Create a failed ability result.
    #[must_use]
    pub fn failed() -> Self {
        Self {
            success: false,
            damage: 0,
            area_radius: 0.0,
            effect: String::new(),
        }
    }
}

/// A hostile creature instance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HostileCreature {
    /// Type of this creature.
    creature_type: HostileType,
    /// Current HP.
    hp: u32,
    /// Maximum HP.
    max_hp: u32,
    /// Attack damage.
    damage: u32,
    /// Movement speed.
    speed: f32,
    /// Special ability name.
    special_ability: String,
    /// Whether the creature is active (not stunned/disabled).
    active: bool,
}

impl HostileCreature {
    /// Create a new hostile creature of the given type.
    #[must_use]
    pub fn new(creature_type: HostileType) -> Self {
        let max_hp = creature_type.base_hp();
        Self {
            creature_type,
            hp: max_hp,
            max_hp,
            damage: creature_type.base_damage(),
            speed: creature_type.base_speed(),
            special_ability: creature_type.special_ability().to_string(),
            active: true,
        }
    }

    /// Get the creature type.
    #[must_use]
    pub fn creature_type(&self) -> HostileType {
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

    /// Get attack damage.
    #[must_use]
    pub fn damage(&self) -> u32 {
        self.damage
    }

    /// Get movement speed.
    #[must_use]
    pub fn speed(&self) -> f32 {
        self.speed
    }

    /// Get the special ability name.
    #[must_use]
    pub fn special_ability(&self) -> &str {
        &self.special_ability
    }

    /// Check if the creature is active.
    #[must_use]
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Set active state.
    pub fn set_active(&mut self, active: bool) {
        self.active = active;
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

    /// Perform a basic attack.
    ///
    /// Returns the damage dealt (0 if inactive or dead).
    #[must_use]
    pub fn attack(&self) -> u32 {
        if self.is_alive() && self.active {
            self.damage
        } else {
            0
        }
    }

    /// Use the creature's special ability.
    #[must_use]
    pub fn use_ability(&self) -> AbilityResult {
        if !self.is_alive() || !self.active {
            return AbilityResult::failed();
        }

        match self.creature_type {
            HostileType::ShadowWalker => AbilityResult::new(
                true,
                self.damage + 10,
                0.0,
                "Attacks from another dimension, bypassing defenses".to_string(),
            ),
            HostileType::MagmaCrawler => AbilityResult::new(
                true,
                self.damage / 2,
                5.0,
                "Splashes lava in an area, dealing burn damage".to_string(),
            ),
            HostileType::CrystalSerpent => AbilityResult::new(
                true,
                self.damage,
                3.0,
                "Shatters into crystal shrapnel on death".to_string(),
            ),
            HostileType::VoidWraith => AbilityResult::new(
                true,
                0,
                8.0,
                "Pulls target toward the void dimension".to_string(),
            ),
            HostileType::NexusBeast => AbilityResult::new(
                true,
                self.damage + 20,
                0.0,
                "Shifts to a different dimension, changing attack pattern".to_string(),
            ),
        }
    }

    /// Heal the creature.
    pub fn heal(&mut self, amount: u32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hostile_type_stats() {
        assert_eq!(HostileType::ShadowWalker.base_hp(), 100);
        assert_eq!(HostileType::ShadowWalker.base_damage(), 18);
        assert!((HostileType::ShadowWalker.base_speed() - 1.5).abs() < f32::EPSILON);
    }

    #[test]
    fn test_hostile_type_nexus_beast_tanky() {
        assert_eq!(HostileType::NexusBeast.base_hp(), 300);
        assert_eq!(HostileType::NexusBeast.base_damage(), 30);
    }

    #[test]
    fn test_hostile_type_special_abilities() {
        assert_eq!(HostileType::ShadowWalker.special_ability(), "phase_strike");
        assert_eq!(HostileType::MagmaCrawler.special_ability(), "lava_splash");
        assert_eq!(HostileType::VoidWraith.special_ability(), "void_pull");
    }

    #[test]
    fn test_hostile_creature_new() {
        let creature = HostileCreature::new(HostileType::CrystalSerpent);

        assert_eq!(creature.creature_type(), HostileType::CrystalSerpent);
        assert_eq!(creature.hp(), 120);
        assert_eq!(creature.max_hp(), 120);
        assert_eq!(creature.damage(), 15);
        assert!((creature.speed() - 2.0).abs() < f32::EPSILON);
        assert!(creature.is_alive());
        assert!(creature.is_active());
    }

    #[test]
    fn test_hostile_creature_take_damage() {
        let mut creature = HostileCreature::new(HostileType::VoidWraith);
        assert_eq!(creature.hp(), 80);

        let dealt = creature.take_damage(30);
        assert_eq!(dealt, 30);
        assert_eq!(creature.hp(), 50);
        assert!(creature.is_alive());
    }

    #[test]
    fn test_hostile_creature_death() {
        let mut creature = HostileCreature::new(HostileType::VoidWraith);
        creature.take_damage(100);

        assert_eq!(creature.hp(), 0);
        assert!(!creature.is_alive());
    }

    #[test]
    fn test_hostile_creature_overkill() {
        let mut creature = HostileCreature::new(HostileType::VoidWraith);
        let dealt = creature.take_damage(1000);

        assert_eq!(dealt, 80); // Only actual HP
        assert_eq!(creature.hp(), 0);
    }

    #[test]
    fn test_hostile_creature_attack() {
        let creature = HostileCreature::new(HostileType::MagmaCrawler);
        assert_eq!(creature.attack(), 22);
    }

    #[test]
    fn test_hostile_creature_attack_when_dead() {
        let mut creature = HostileCreature::new(HostileType::MagmaCrawler);
        creature.take_damage(200);
        assert_eq!(creature.attack(), 0);
    }

    #[test]
    fn test_hostile_creature_attack_when_inactive() {
        let mut creature = HostileCreature::new(HostileType::MagmaCrawler);
        creature.set_active(false);
        assert_eq!(creature.attack(), 0);
    }

    #[test]
    fn test_hostile_creature_use_ability() {
        let creature = HostileCreature::new(HostileType::ShadowWalker);
        let result = creature.use_ability();

        assert!(result.success);
        assert_eq!(result.damage, 28); // base 18 + 10
        assert!(!result.effect.is_empty());
    }

    #[test]
    fn test_hostile_creature_ability_when_dead() {
        let mut creature = HostileCreature::new(HostileType::ShadowWalker);
        creature.take_damage(200);
        let result = creature.use_ability();

        assert!(!result.success);
    }

    #[test]
    fn test_hostile_creature_heal() {
        let mut creature = HostileCreature::new(HostileType::NexusBeast);
        creature.take_damage(100);
        assert_eq!(creature.hp(), 200);

        creature.heal(50);
        assert_eq!(creature.hp(), 250);

        creature.heal(100);
        assert_eq!(creature.hp(), 300); // Capped at max
    }

    #[test]
    fn test_ability_result_failed() {
        let result = AbilityResult::failed();
        assert!(!result.success);
        assert_eq!(result.damage, 0);
    }

    #[test]
    fn test_magma_crawler_area_ability() {
        let creature = HostileCreature::new(HostileType::MagmaCrawler);
        let result = creature.use_ability();

        assert!(result.success);
        assert!(result.area_radius > 0.0);
    }
}
