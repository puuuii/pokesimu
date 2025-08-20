use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub is_main_series: bool,
    pub generation: GenerationReference,
    pub names: Vec<Name>,
    pub effect_entries: Vec<EffectEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationReference {
    pub name: Generation,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub language: LanguageReference,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageReference {
    pub name: Language,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub language: LanguageReference,
    pub short_effect: String,
}

// Re-use enums from moves domain
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
