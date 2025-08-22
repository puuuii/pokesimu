use super::common::{
    GenerationReference, GrowthRate, LanguageReference, Name, PalParkArea, PokemonColor,
    PokemonHabitat, PokemonReference, PokemonShape, VersionReference,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenderRate {
    Genderless,     // -1
    AlwaysMale,     // 0
    Mostly1Female7, // 1 (1/8 female, 7/8 male)
    Mostly2Female6, // 2 (2/8 female, 6/8 male)
    Mostly3Female5, // 3 (3/8 female, 5/8 male)
    Equal,          // 4 (4/8 female, 4/8 male)
    Mostly5Female3, // 5 (5/8 female, 3/8 male)
    Mostly6Female2, // 6 (6/8 female, 2/8 male)
    Mostly7Female1, // 7 (7/8 female, 1/8 male)
    AlwaysFemale,   // 8
}

impl GenderRate {
    pub fn from_i32(value: i32) -> Result<Self, String> {
        match value {
            -1 => Ok(GenderRate::Genderless),
            0 => Ok(GenderRate::AlwaysMale),
            1 => Ok(GenderRate::Mostly1Female7),
            2 => Ok(GenderRate::Mostly2Female6),
            3 => Ok(GenderRate::Mostly3Female5),
            4 => Ok(GenderRate::Equal),
            5 => Ok(GenderRate::Mostly5Female3),
            6 => Ok(GenderRate::Mostly6Female2),
            7 => Ok(GenderRate::Mostly7Female1),
            8 => Ok(GenderRate::AlwaysFemale),
            _ => Err(format!("Invalid gender_rate value: {}", value)),
        }
    }

    pub fn to_i32(&self) -> i32 {
        match self {
            GenderRate::Genderless => -1,
            GenderRate::AlwaysMale => 0,
            GenderRate::Mostly1Female7 => 1,
            GenderRate::Mostly2Female6 => 2,
            GenderRate::Mostly3Female5 => 3,
            GenderRate::Equal => 4,
            GenderRate::Mostly5Female3 => 5,
            GenderRate::Mostly6Female2 => 6,
            GenderRate::Mostly7Female1 => 7,
            GenderRate::AlwaysFemale => 8,
        }
    }

    pub fn female_probability(&self) -> Option<f64> {
        match self {
            GenderRate::Genderless => None,
            GenderRate::AlwaysMale => Some(0.0),
            GenderRate::Mostly1Female7 => Some(1.0 / 8.0),
            GenderRate::Mostly2Female6 => Some(2.0 / 8.0),
            GenderRate::Mostly3Female5 => Some(3.0 / 8.0),
            GenderRate::Equal => Some(4.0 / 8.0),
            GenderRate::Mostly5Female3 => Some(5.0 / 8.0),
            GenderRate::Mostly6Female2 => Some(6.0 / 8.0),
            GenderRate::Mostly7Female1 => Some(7.0 / 8.0),
            GenderRate::AlwaysFemale => Some(1.0),
        }
    }
}

impl<'de> serde::de::Deserialize<'de> for GenderRate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        GenderRate::from_i32(value).map_err(serde::de::Error::custom)
    }
}

impl serde::ser::Serialize for GenderRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        self.to_i32().serialize(serializer)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpecies {
    pub id: u32,
    pub name: String,
    pub base_happiness: u32,
    pub capture_rate: u32,
    pub color: PokemonColorReference,
    pub egg_groups: Vec<EggGroupReference>,
    pub evolution_chain: EvolutionChainReference,
    pub evolves_from_species: Option<PokemonSpeciesReference>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub form_descriptions: Vec<FormDescription>,
    pub forms_switchable: bool,
    pub gender_rate: GenderRate,
    pub genera: Vec<Genus>,
    pub generation: GenerationReference,
    pub growth_rate: GrowthRateReference,
    pub habitat: Option<PokemonHabitatReference>,
    pub has_gender_differences: bool,
    pub hatch_counter: u32,
    pub is_baby: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub names: Vec<Name>,
    pub order: u32,
    pub pal_park_encounters: Vec<PalParkEncounter>,
    pub pokedex_numbers: Vec<PokedexNumber>,
    pub shape: PokemonShapeReference,
    pub varieties: Vec<PokemonVariety>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonColorReference {
    pub name: PokemonColor,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonShapeReference {
    pub name: PokemonShape,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonHabitatReference {
    pub name: PokemonHabitat,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrowthRateReference {
    pub name: GrowthRate,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PalParkAreaReference {
    pub name: PalParkArea,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EggGroupReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvolutionChainReference {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonSpeciesReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: LanguageReference,
    pub version: VersionReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FormDescription {
    pub description: String,
    pub language: LanguageReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Genus {
    pub genus: String,
    pub language: LanguageReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PalParkEncounter {
    pub area: PalParkAreaReference,
    pub base_score: u32,
    pub rate: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokedexNumber {
    pub entry_number: u32,
    pub pokedex: PokedexReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokedexReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PokemonVariety {
    pub is_default: bool,
    pub pokemon: PokemonReference,
}
