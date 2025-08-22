use super::common::{
    GenerationReference, PokemonType as PokemonTypeEnum, Version, VersionGroupReference,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PokemonOrder {
    Normal(u32),
    Special, // -1 を表現
}

impl PokemonOrder {
    pub fn from_i32(value: i32) -> Self {
        if value == -1 {
            PokemonOrder::Special
        } else {
            PokemonOrder::Normal(value as u32)
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            PokemonOrder::Normal(n) => *n as i32,
            PokemonOrder::Special => -1,
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for PokemonOrder {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        Ok(PokemonOrder::from_i32(value))
    }
}

impl serde::ser::Serialize for PokemonOrder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_i32().serialize(serializer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub base_experience: Option<u32>,
    pub height: u32,
    pub is_default: bool,
    pub order: PokemonOrder,
    pub weight: u32,
    pub abilities: Vec<PokemonAbility>,
    pub forms: Vec<PokemonFormReference>,
    pub game_indices: Vec<PokemonGameIndex>,
    pub held_items: Vec<PokemonHeldItem>,
    pub location_area_encounters: String,
    pub moves: Vec<PokemonMove>,
    pub past_types: Vec<PokemonTypePast>,
    pub sprites: PokemonSprites,
    pub species: PokemonSpeciesReference,
    pub stats: Vec<PokemonStat>,
    pub types: Vec<PokemonTypeSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: u32,
    pub ability: AbilityReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AbilityReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonFormReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonGameIndex {
    pub game_index: u32,
    pub version: VersionReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionReference {
    pub name: Version,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonHeldItem {
    pub item: ItemReference,
    pub version_details: Vec<PokemonHeldItemVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonHeldItemVersion {
    pub rarity: u32,
    pub version: VersionReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    pub move_ref: MoveReference,
    pub version_group_details: Vec<PokemonMoveVersion>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonMoveVersion {
    pub level_learned_at: u32,
    pub move_learn_method: MoveLearnMethodReference,
    pub version_group: VersionGroupReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MoveLearnMethodReference {
    pub name: MoveLearnMethod,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonTypePast {
    pub generation: GenerationReference,
    pub types: Vec<PokemonTypeSlot>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSprites {
    pub back_default: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny: Option<String>,
    pub back_shiny_female: Option<String>,
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
    pub other: Option<PokemonSpritesOther>,
    pub versions: Option<PokemonSpritesVersions>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpritesOther {
    pub dream_world: Option<PokemonSpritesDreamWorld>,
    pub home: Option<PokemonSpritesHome>,
    #[serde(rename = "official-artwork")]
    pub official_artwork: Option<PokemonSpritesOfficialArtwork>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpritesDreamWorld {
    pub front_default: Option<String>,
    pub front_female: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpritesHome {
    pub front_default: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny: Option<String>,
    pub front_shiny_female: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpritesOfficialArtwork {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpritesVersions {
    #[serde(rename = "generation-i")]
    pub generation_i: Option<PokemonSpritesVersionGeneration>,
    #[serde(rename = "generation-ii")]
    pub generation_ii: Option<PokemonSpritesVersionGeneration>,
    #[serde(rename = "generation-iii")]
    pub generation_iii: Option<PokemonSpritesVersionGeneration>,
    #[serde(rename = "generation-iv")]
    pub generation_iv: Option<PokemonSpritesVersionGeneration>,
    #[serde(rename = "generation-v")]
    pub generation_v: Option<PokemonSpritesVersionGeneration>,
    #[serde(rename = "generation-vi")]
    pub generation_vi: Option<PokemonSpritesVersionGeneration>,
    #[serde(rename = "generation-vii")]
    pub generation_vii: Option<PokemonSpritesVersionGeneration>,
    #[serde(rename = "generation-viii")]
    pub generation_viii: Option<PokemonSpritesVersionGeneration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpritesVersionGeneration {
    #[serde(flatten)]
    pub games: std::collections::HashMap<String, PokemonSpritesGame>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpritesGame {
    pub back_default: Option<String>,
    pub back_gray: Option<String>,
    pub back_transparent: Option<String>,
    pub front_default: Option<String>,
    pub front_gray: Option<String>,
    pub front_transparent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpeciesReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonStat {
    pub base_stat: u32,
    pub effort: u32,
    pub stat: StatReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatReference {
    pub name: Stat,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonTypeSlot {
    pub slot: u32,
    #[serde(rename = "type")]
    pub type_ref: TypeReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypeReference {
    pub name: PokemonTypeEnum,
    pub url: String,
}

// ENUMs

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MoveLearnMethod {
    Egg,
    LevelUp,
    Machine,
    Reminder,
    Tutor,
    LightBallEgg,
    ColosseumPurification,
    XdShadow,
    XdPurification,
    FormChange,
    ZygardeCube,
    StadiumSurfingPikachu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Stat {
    Hp,
    Attack,
    Defense,
    SpecialAttack,
    SpecialDefense,
    Speed,
    Accuracy,
    Evasion,
}
