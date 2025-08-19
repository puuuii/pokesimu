use serde_json;
use std::fs;
use std::path::PathBuf;

use crate::domain::ability::Ability;
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
        println!("{:#?}", results);
    }
}
