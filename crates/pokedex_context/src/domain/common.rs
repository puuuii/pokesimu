#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeReference {
    pub name: PokemonType,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeMove {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypePokemonReference {
    pub name: String,
    pub url: String,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageReference {
    pub name: Language,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenerationReference {
    pub name: Generation,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionGroupReference {
    pub name: VersionGroup,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    pub language: LanguageReference,
}

/// 基本的なEffectEntry（movesで使用する柔軟なバージョン）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub language: LanguageReference,
    pub short_effect: Option<String>,
}

/// 必須のshort_effectを持つEffectEntry（abilityとitemで使用）
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequiredShortEffectEntry {
    pub effect: String,
    pub language: LanguageReference,
    pub short_effect: String,
}

/// 共通のenum定義

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
    Roomaji,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "zh-Hant")]
    ZhHant,
}

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
#[serde(rename_all = "kebab-case")]
pub enum VersionGroup {
    #[serde(rename = "black-2-white-2")]
    Black2White2,
    BlackWhite,
    Blue,
    #[serde(rename = "brilliant-diamond-and-shining-pearl")]
    BrilliantDiamondAndShiningPearl,
    Colosseum,
    Crystal,
    Diamond,
    DiamondPearl,
    Emerald,
    Firered,
    FireredLeafgreen,
    Gold,
    GoldSilver,
    Heartgold,
    HeartgoldSoulsilver,
    Leafgreen,
    #[serde(rename = "legends-arceus")]
    LegendsArceus,
    #[serde(rename = "lets-go-eevee")]
    LetsGoEevee,
    #[serde(rename = "lets-go-pikachu")]
    LetsGoPikachu,
    #[serde(rename = "lets-go-pikachu-lets-go-eevee")]
    LetsGoPikachuLetsGoEevee,
    Moon,
    OmegaRuby,
    OmegaRubyAlphaSapphire,
    Pearl,
    Platinum,
    Red,
    #[serde(rename = "red-blue")]
    RedBlue,
    Ruby,
    RubySapphire,
    Sapphire,
    Scarlet,
    #[serde(rename = "scarlet-violet")]
    ScarletViolet,
    Shield,
    Silver,
    Soulsilver,
    Sun,
    SunMoon,
    Sword,
    SwordShield,
    #[serde(rename = "the-indigo-disk")]
    TheIndigoDisk,
    #[serde(rename = "the-teal-mask")]
    TheTealMask,
    UltraMoon,
    UltraSun,
    #[serde(rename = "ultra-sun-ultra-moon")]
    UltraSunUltraMoon,
    Violet,
    White,
    #[serde(rename = "white-2")]
    White2,
    X,
    Xd,
    #[serde(rename = "x-y")]
    XY,
    Y,
    Yellow,
}

// Version enum - extended with missing variants
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Version {
    AlphaSapphire,
    #[serde(rename = "black-2")]
    Black2,
    Black,
    Black2White2,
    BlackWhite,
    Blue,
    BrilliantDiamondAndShiningPearl,
    Colosseum,
    Crystal,
    Diamond,
    DiamondPearl,
    Emerald,
    Firered,
    FireredLeafgreen,
    Gold,
    GoldSilver,
    Heartgold,
    HeartgoldSoulsilver,
    Leafgreen,
    LegendsArceus,
    #[serde(rename = "lets-go-eevee")]
    LetsGoEevee,
    #[serde(rename = "lets-go-pikachu")]
    LetsGoPikachu,
    LetsGoPikachuLetsGoEevee,
    Moon,
    OmegaRuby,
    OmegaRubyAlphaSapphire,
    Pearl,
    Platinum,
    Red,
    RedBlue,
    Ruby,
    RubySapphire,
    Sapphire,
    Scarlet,
    ScarletViolet,
    Shield,
    Silver,
    Soulsilver,
    Sun,
    SunMoon,
    Sword,
    SwordShield,
    TheIndigoDisk,
    TheTealMask,
    #[serde(rename = "ultra-moon")]
    UltraMoon,
    UltraSun,
    UltraSunUltraMoon,
    Violet,
    White,
    #[serde(rename = "white-2")]
    White2,
    Xd,
    X,
    #[serde(rename = "x-y")]
    XY,
    Y,
    Yellow,
}

// 共通のPokemonType enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PokemonType {
    Bug,
    Dark,
    Dragon,
    Electric,
    Fairy,
    Fighting,
    Fire,
    Flying,
    Ghost,
    Grass,
    Ground,
    Ice,
    Normal,
    Poison,
    Psychic,
    Rock,
    Shadow,
    Stellar,
    Steel,
    Unknown,
    Water,
}

// Pokemon-related enums
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PokemonColor {
    Black,
    Blue,
    Brown,
    Gray,
    Green,
    Pink,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PokemonShape {
    Armor,
    Arms,
    Ball,
    Blob,
    #[serde(rename = "bug-wings")]
    BugWings,
    Fish,
    Heads,
    Humanoid,
    Legs,
    Quadruped,
    Squiggle,
    Tentacles,
    Upright,
    Wings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PokemonHabitat {
    Cave,
    Forest,
    Grassland,
    Mountain,
    Rare,
    #[serde(rename = "rough-terrain")]
    RoughTerrain,
    Sea,
    Urban,
    #[serde(rename = "waters-edge")]
    WatersEdge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GrowthRate {
    Fast,
    #[serde(rename = "fast-then-very-slow")]
    FastThenVerySlow,
    Medium,
    #[serde(rename = "medium-slow")]
    MediumSlow,
    Slow,
    #[serde(rename = "slow-then-very-fast")]
    SlowThenVeryFast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PalParkArea {
    Field,
    Forest,
    Mountain,
    Pond,
    Sea,
}

// Common reference structures
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionReference {
    pub name: Version,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonReference {
    pub name: String,
    pub url: String,
}
