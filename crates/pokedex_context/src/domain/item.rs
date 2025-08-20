use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub cost: u32,
    pub category: ItemCategoryReference,
    pub attributes: Vec<ItemAttributeReference>,
    pub effect_entries: Vec<EffectEntry>,
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
pub struct EffectEntry {
    pub effect: String,
    #[serde(rename = "short_effect")]
    pub short_effect: String,
    pub language: LanguageReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub text: String,
    pub language: LanguageReference,
    #[serde(rename = "version_group")]
    pub version_group: VersionGroupReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    pub language: LanguageReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageReference {
    pub name: Language,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGroupReference {
    pub name: VersionGroup,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemSprites {
    pub default: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlingEffect {
    pub name: String,
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
    pub version: Version,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameIndex {
    #[serde(rename = "game_index")]
    pub game_index: u32,
    pub generation: GenerationReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationReference {
    pub name: Generation,
    pub url: String,
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

// Re-use enums from moves/ability domain
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Generation {
    #[serde(rename = "generation-i")]
    GenerationI,
    #[serde(rename = "generation-ii")]
    GenerationII,
    #[serde(rename = "generation-iii")]
    GenerationIII,
    #[serde(rename = "generation-iv")]
    GenerationIV,
    #[serde(rename = "generation-v")]
    GenerationV,
    #[serde(rename = "generation-vi")]
    GenerationVI,
    #[serde(rename = "generation-vii")]
    GenerationVII,
    #[serde(rename = "generation-viii")]
    GenerationVIII,
    #[serde(rename = "generation-ix")]
    GenerationIX,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    De,
    En,
    Es,
    Fr,
    It,
    Ja,
    #[serde(rename = "ja-Hrkt")]
    JaHrkt,
    Ko,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "zh-Hant")]
    ZhHant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum VersionGroup {
    #[serde(rename = "black-2-white-2")]
    Black2White2,
    BlackWhite,
    Crystal,
    DiamondPearl,
    Emerald,
    FireredLeafgreen,
    GoldSilver,
    HeartgoldSoulsilver,
    #[serde(rename = "lets-go-pikachu-lets-go-eevee")]
    LetsGoPikachuLetsGoEevee,
    Platinum,
    RubySapphire,
    SunMoon,
    SwordShield,
    #[serde(rename = "ultra-sun-ultra-moon")]
    UltraSunUltraMoon,
    XY,
}
