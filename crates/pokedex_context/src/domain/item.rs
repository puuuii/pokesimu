use super::common::{
    GenerationReference, LanguageReference, Name, RequiredShortEffectEntry, Version,
    VersionGroupReference,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub cost: u32,
    pub category: ItemCategoryReference,
    pub attributes: Vec<ItemAttributeReference>,
    pub effect_entries: Vec<RequiredShortEffectEntry>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub names: Vec<Name>,
    pub sprites: ItemSprites,
    pub fling_power: Option<u32>,
    pub fling_effect: Option<FlingEffect>,
    pub baby_trigger_for: Option<BabyTriggerFor>,
    pub held_by_pokemon: Vec<HeldByPokemon>,
    pub machines: Vec<Machine>,
    pub game_indices: Vec<GameIndex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemCategoryReference {
    pub name: ItemCategory,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAttributeReference {
    pub name: ItemAttribute,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub text: String,
    pub language: LanguageReference,
    #[serde(rename = "version_group")]
    pub version_group: VersionGroupReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSprites {
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlingEffect {
    pub name: FlingEffectType,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeldByPokemon {
    pub pokemon: PokemonReference,
    #[serde(rename = "version_details")]
    pub version_details: Vec<VersionDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PokemonReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDetail {
    pub rarity: u32,
    pub version: VersionReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionReference {
    pub name: Version,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameIndex {
    #[serde(rename = "game_index")]
    pub game_index: u32,
    pub generation: GenerationReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub machine: MachineReference,
    #[serde(rename = "version_group")]
    pub version_group: VersionGroupReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineReference {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BabyTriggerFor {
    pub url: String,
}

// ItemCategory enum - 54 variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ItemCategory {
    AllMachines,
    AllMail,
    ApricornBalls,
    ApricornBox,
    BadHeldItems,
    BakingOnly,
    CatchingBonus,
    Choice,
    Collectibles,
    CurryIngredients,
    DataCards,
    DexCompletion,
    DynamaxCrystals,
    EffortDrop,
    EffortTraining,
    EventItems,
    Evolution,
    Flutes,
    Gameplay,
    Healing,
    HeldItems,
    InAPinch,
    Jewels,
    Loot,
    Medicine,
    MegaStones,
    Memories,
    MiracleShooter,
    Mulch,
    NatureMints,
    Other,
    PickyHealing,
    Picnic,
    Plates,
    PlotAdvancement,
    PpRecovery,
    Revival,
    SandwichIngredients,
    Scarves,
    SpecialBalls,
    SpeciesCandies,
    SpeciesSpecific,
    Spelunking,
    StandardBalls,
    StatBoosts,
    StatusCures,
    TeraShard,
    TmMaterials,
    Training,
    TypeEnhancement,
    TypeProtection,
    Unused,
    Vitamins,
    ZCrystals,
}

// ItemAttribute enum - 7 variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ItemAttribute {
    Consumable,
    Countable,
    Holdable,
    HoldableActive,
    Underground,
    UsableInBattle,
    UsableOverworld,
}

// FlingEffectType enum - 7 variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FlingEffectType {
    BadlyPoison,
    BerryEffect,
    Burn,
    Flinch,
    HerbEffect,
    Paralyze,
    Poison,
}
