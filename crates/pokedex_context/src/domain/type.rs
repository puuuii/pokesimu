use super::common::{Name, PokemonType, TypeMove, TypePokemonReference, TypeReference};
use serde::{Deserialize, Serialize};

/// Typeのバリューオブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Type {
    pub id: u32,
    pub name: PokemonType,
    pub names: Vec<Name>,
    pub damage_relations: DamageRelations,
    pub pokemon: Vec<TypePokemon>,
    pub moves: Vec<TypeMove>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DamageRelations {
    pub double_damage_from: Vec<TypeReference>,
    pub double_damage_to: Vec<TypeReference>,
    pub half_damage_from: Vec<TypeReference>,
    pub half_damage_to: Vec<TypeReference>,
    pub no_damage_from: Vec<TypeReference>,
    pub no_damage_to: Vec<TypeReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypePokemon {
    pub slot: u32,
    pub pokemon: TypePokemonReference,
}
