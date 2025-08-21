use super::common::{
    GenerationReference, GrowthRate, LanguageReference, Name, PalParkArea, PokemonColor,
    PokemonHabitat, PokemonReference, PokemonShape, VersionReference,
};
use serde::{Deserialize, Serialize};

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
    pub gender_rate: i32,
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
