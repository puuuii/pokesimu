use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub is_main_series: bool,
    pub generation: Generation,
    pub names: Vec<Name>,
    pub effect_entries: Vec<EffectEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub language: Language,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    pub language: Language,
    pub short_effect: String,
}
