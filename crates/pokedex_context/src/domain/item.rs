use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub cost: u32,
    pub category: ItemCategory,
    pub attributes: Vec<ItemAttribute>,
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
pub struct ItemCategory {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemAttribute {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectEntry {
    pub effect: String,
    #[serde(rename = "short_effect")]
    pub short_effect: String,
    pub language: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub text: String,
    pub language: Language,
    #[serde(rename = "version_group")]
    pub version_group: VersionGroup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name {
    pub name: String,
    pub language: Language,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionGroup {
    pub name: String,
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
    pub generation: Generation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generation {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Machine {
    pub machine: MachineReference,
    #[serde(rename = "version_group")]
    pub version_group: VersionGroup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineReference {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BabyTriggerFor {
    pub url: String,
}
