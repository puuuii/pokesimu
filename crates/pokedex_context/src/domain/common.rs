use serde::{Deserialize, Serialize};

/// 共通のreference構造体

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
    Colosseum,
    Crystal,
    DiamondPearl,
    Emerald,
    FireredLeafgreen,
    GoldSilver,
    HeartgoldSoulsilver,
    #[serde(rename = "lets-go-pikachu-lets-go-eevee")]
    LetsGoPikachuLetsGoEevee,
    #[serde(rename = "legends-arceus")]
    LegendsArceus,
    OmegaRubyAlphaSapphire,
    Platinum,
    RubySapphire,
    #[serde(rename = "scarlet-violet")]
    ScarletViolet,
    SunMoon,
    SwordShield,
    #[serde(rename = "ultra-sun-ultra-moon")]
    UltraSunUltraMoon,
    #[serde(rename = "x-y")]
    XY,
}

// Version enum - 22 variants
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum Version {
    AlphaSapphire,
    Black,
    Black2,
    Black2White2,
    BlackWhite,
    BrilliantDiamondAndShiningPearl,
    Colosseum,
    Crystal,
    Diamond,
    DiamondPearl,
    Emerald,
    Firered,
    FireredLeafgreen,
    GoldSilver,
    Heartgold,
    HeartgoldSoulsilver,
    Leafgreen,
    LegendsArceus,
    LetsGoPikachuLetsGoEevee,
    Moon,
    OmegaRuby,
    OmegaRubyAlphaSapphire,
    Pearl,
    Platinum,
    RedBlue,
    Ruby,
    RubySapphire,
    Sapphire,
    ScarletViolet,
    Soulsilver,
    Sun,
    SunMoon,
    SwordShield,
    TheIndigoDisk,
    TheTealMask,
    UltraSun,
    UltraSunUltraMoon,
    White,
    White2,
    X,
    XY,
    Y,
}
