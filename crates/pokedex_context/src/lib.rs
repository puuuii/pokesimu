pub mod domain {
    pub mod ability;
    pub mod common;
    pub mod item;
    pub mod moves;
    pub mod pokemon;
    pub mod repositories;
}
pub mod infrastructure {
    pub mod persistence {
        pub mod json_pokedex_repository;
    }
}
