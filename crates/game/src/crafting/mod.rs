//! Crafting system with recipes and execution.
//!
//! Includes standard crafting as well as Fracture-specific crafting stations:
//! Reality Forge, Transmutation Table, Stability Infuser, and Anchor Workshop.

mod anchor_workshop;
mod executor;
mod furnace;
mod reality_forge;
mod registry;
mod stability_infuser;
mod transmutation_table;

pub use anchor_workshop::{anchor_build_costs, AnchorWorkshop};
pub use executor::{check_craft, execute_craft, execute_craft_by_id, CraftError, CraftRequirements};
pub use furnace::{
    Furnace, FurnaceState, FuelEntry, DEFAULT_SMELT_TIME, FUEL_CHARCOAL, FUEL_COAL,
    FUEL_LAVA_BUCKET, FUEL_STICK, FUEL_WOOD,
};
pub use reality_forge::{CraftResult, ForgeRecipe, RealityForge};
pub use registry::{CraftingStation, Ingredient, Recipe, RecipeRegistry};
pub use stability_infuser::{StabilityInfuser, ENERGY_PER_LEVEL, MIN_INFUSION_ENERGY};
pub use transmutation_table::TransmutationCraftingTable;
