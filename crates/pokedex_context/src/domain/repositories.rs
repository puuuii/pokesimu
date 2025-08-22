use crate::domain::ability::Ability;
use crate::domain::item::Item;
use crate::domain::moves::Move;
use crate::domain::pokemon::Pokemon;
use crate::domain::pokemon_species::PokemonSpecies;
use crate::domain::r#type::Type;

pub trait IPokedexRepository {
    fn load_all_abilities(&self) -> Vec<Ability>;
    fn load_all_items(&self) -> Vec<Item>;
    fn load_all_moves(&self) -> Vec<Move>;
    fn load_all_pokemon(&self) -> Vec<Pokemon>;
    fn load_all_pokemon_species(&self) -> Vec<PokemonSpecies>;
    fn load_all_types(&self) -> Vec<Type>;
}
