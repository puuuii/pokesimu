use serde_json;
use std::fs;
use std::path::PathBuf;

use crate::domain::ability::Ability;
use crate::domain::item::Item;
use crate::domain::moves::Move;
use crate::domain::repositories::IPokedexRepository;

pub struct JsonPokedexRepository {
    data_path: PathBuf,
}

impl JsonPokedexRepository {
    pub fn new(data_path: PathBuf) -> Self {
        JsonPokedexRepository { data_path }
    }
}

impl IPokedexRepository for JsonPokedexRepository {
    fn load_all_abilities(&self) -> Vec<Ability> {
        let ability_dir = self.data_path.join("ability");
        let entries = fs::read_dir(&ability_dir).unwrap();
        let mut abilities = Vec::new();

        for entry in entries {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let data = fs::read_to_string(&path).unwrap();
                match serde_json::from_str::<Ability>(&data) {
                    Ok(ability) => abilities.push(ability),
                    Err(e) => {
                        eprintln!("Failed to parse ability file {:?}: {}", path, e);
                    }
                }
            }
        }

        abilities
    }

    fn load_all_items(&self) -> Vec<Item> {
        let item_dir = self.data_path.join("item");
        let entries = fs::read_dir(&item_dir).unwrap();
        let mut items = Vec::new();

        for entry in entries {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let data = fs::read_to_string(&path).unwrap();
                match serde_json::from_str::<Item>(&data) {
                    Ok(item) => items.push(item),
                    Err(e) => {
                        eprintln!("Failed to parse item file {:?}: {}", path, e);
                    }
                }
            }
        }

        items
    }

    fn load_all_moves(&self) -> Vec<Move> {
        let move_dir = self.data_path.join("move");
        let entries = fs::read_dir(&move_dir).unwrap();
        let mut moves = Vec::new();

        for entry in entries {
            let path = entry.unwrap().path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let data = fs::read_to_string(&path).unwrap();
                match serde_json::from_str::<Move>(&data) {
                    Ok(move_data) => moves.push(move_data),
                    Err(e) => {
                        eprintln!("Failed to parse move file {:?}: {}", path, e);
                    }
                }
            }
        }

        moves
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
}
