use pokedex_context::domain::repositories::IPokedexRepository;
use pokedex_context::infrastructure::persistence::json_pokedex_repository::JsonPokedexRepository;
use std::path::PathBuf;

fn main() {
    let data_path = PathBuf::from("data/");
    let repository = JsonPokedexRepository::new(data_path);

    let items = repository.load_all_items();
    println!("Successfully loaded {} items", items.len());

    let moves = repository.load_all_moves();
    println!("Successfully loaded {} moves", moves.len());

    let pokemon = repository.load_all_pokemon();
    println!("Successfully loaded {} pokemon", pokemon.len());

    let pokemon_species = repository.load_all_pokemon_species();
    println!(
        "Successfully loaded {} pokemon species",
        pokemon_species.len()
    );

    let types = repository.load_all_types();
    println!("Successfully loaded {} types", types.len());
}
