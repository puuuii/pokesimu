# pokesimu

## rayout

```txt
├── Cargo.toml
├── data/
│   ├── pokemon
│   └── moves
│
├── crates/
│   ├── battle_context/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       │
│   │       ├── domain.rs          # 変更点: domain/mod.rs -> domain.rs
│   │       ├── domain/
│   │       │   ├── battle.rs
│   │       │   ├── pokemon_in_battle.rs
│   │       │   ├── value_objects.rs # 変更点: value_objects/mod.rs -> value_objects.rs
│   │       │   └── value_objects/
│   │       │       ├── hp.rs
│   │       │       └── stats.rs
│   │       │
│   │       ├── application.rs     # 変更点: application/mod.rs -> application.rs
│   │       ├── application/
│   │       │   ├── use_cases.rs     # 変更点: use_cases/mod.rs -> use_cases.rs
│   │       │   ├── use_cases/
│   │       │   │   ├── create_battle.rs
│   │       │   │   └── select_pokemon.rs
│   │       │   └── battle_service.rs
│   │       │
│   │       └── infrastructure.rs  # 変更点: infrastructure/mod.rs -> infrastructure.rs
│   │           └── infrastructure/
│   │               ├── persistence.rs
│   │               └── persistence/
│   │                   └── in_memory_battle_repository.rs
│   │
│   ├── pokedex_context/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── domain.rs
│   │       ├── domain/
│   │       │   ├── pokedex.rs
│   │       │   └── repositories.rs
│   │       │
│   │       ├── application.rs
│   │       ├── application/
│   │       │   └── pokedex_service.rs
│   │       │
│   │       └── infrastructure.rs
│   │           └── infrastructure/
│   │               ├── persistence.rs
│   │               └── persistence/
│   │                   └── json_pokedex_repository.rs
│   │
│   └── common_domain/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           ├── pokemon_type.rs
│           └── nature.rs
│
└── src/
    └── main.rs
```
