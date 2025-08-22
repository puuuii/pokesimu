use serde::de::DeserializeOwned;
use serde_json;
use std::fs;
use std::path::PathBuf;

use crate::domain::ability::Ability;
use crate::domain::item::Item;
use crate::domain::moves::Move;
use crate::domain::pokemon::Pokemon;
use crate::domain::pokemon_species::PokemonSpecies;
use crate::domain::repositories::IPokedexRepository;
use crate::domain::r#type::Type;

pub struct JsonPokedexRepository {
    data_path: PathBuf,
}

impl JsonPokedexRepository {
    pub fn new(data_path: PathBuf) -> Self {
        JsonPokedexRepository { data_path }
    }

    fn load_all_from_directory<T>(&self, directory_name: &str) -> Vec<T>
    where
        T: DeserializeOwned,
    {
        let dir = self.data_path.join(directory_name);
        let entries = fs::read_dir(&dir).unwrap();
        let mut results = Vec::new();

        for entry in entries {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let data = fs::read_to_string(&path).unwrap();
                match serde_json::from_str::<T>(&data) {
                    Ok(item) => results.push(item),
                    Err(e) => {
                        eprintln!("Failed to parse {} file {:?}: {}", directory_name, path, e);
                    }
                }
            }
        }

        results
    }
}

impl IPokedexRepository for JsonPokedexRepository {
    fn load_all_abilities(&self) -> Vec<Ability> {
        self.load_all_from_directory("ability")
    }

    fn load_all_items(&self) -> Vec<Item> {
        self.load_all_from_directory("item")
    }

    fn load_all_moves(&self) -> Vec<Move> {
        self.load_all_from_directory("move")
    }

    fn load_all_pokemon(&self) -> Vec<Pokemon> {
        self.load_all_from_directory("pokemon")
    }

    fn load_all_pokemon_species(&self) -> Vec<PokemonSpecies> {
        self.load_all_from_directory("pokemon-species")
    }

    fn load_all_types(&self) -> Vec<Type> {
        self.load_all_from_directory("type")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_all_abilities() {
        // given
        let data_path = PathBuf::from("../../data/");
        let repository = JsonPokedexRepository::new(data_path);

        // when
        let results = repository.load_all_abilities();

        // then
        println!("Successfully loaded {} abilities", results.len());
        println!("{:#?}", results);
    }

    #[test]
    fn test_load_all_items() {
        // given
        let data_path = PathBuf::from("../../data/");
        let repository = JsonPokedexRepository::new(data_path);

        // when
        let results = repository.load_all_items();

        // then
        println!("Successfully loaded {} items", results.len());

        println!("{:#?}", results[200]);
    }

    #[test]
    fn test_load_all_types() {
        // given
        let data_path = PathBuf::from("../../data/");
        let repository = JsonPokedexRepository::new(data_path);

        // when
        let results = repository.load_all_types();

        // then
        println!("Successfully loaded {} types", results.len());
        println!("{:#?}", results[10]);
    }

    #[test]
    fn test_load_all_moves() {
        // given
        let data_path = PathBuf::from("../../data/");
        let repository = JsonPokedexRepository::new(data_path);

        // when
        let results = repository.load_all_moves();

        // then
        println!("Successfully loaded {} moves", results.len());
        println!("{:#?}", results[50]);
    }

    #[test]
    fn test_load_all_pokemon() {
        // given
        let data_path = PathBuf::from("../../data/");
        let repository = JsonPokedexRepository::new(data_path);

        // when
        let results = repository.load_all_pokemon();

        // then
        println!("Successfully loaded {} pokemon", results.len());
        println!("{:#?}", results[100]);
    }

    #[test]
    fn test_load_all_pokemon_species() {
        // given
        let data_path = PathBuf::from("../../data/");
        let repository = JsonPokedexRepository::new(data_path);

        // when
        let results = repository.load_all_pokemon_species();

        // then
        println!("Successfully loaded {} pokemon species", results.len());
        println!("{:#?}", results[100]);
    }
}
