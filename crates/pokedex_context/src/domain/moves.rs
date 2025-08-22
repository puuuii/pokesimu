use super::common::{
    EffectEntry, GenerationReference, LanguageReference, Name, VersionGroupReference,
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Move {
    pub accuracy: Option<u8>,
    pub contest_combos: Option<ContestCombos>,
    pub contest_effect: Option<ContestEffectReference>,
    pub contest_type: Option<ContestTypeReference>,
    pub damage_class: DamageClassReference,
    pub effect_chance: Option<u8>,
    pub effect_changes: Vec<EffectChange>,
    pub effect_entries: Vec<EffectEntry>,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub generation: GenerationReference,
    pub id: u32,
    pub learned_by_pokemon: Vec<PokemonReference>,
    pub machines: Vec<MachineEntry>,
    pub meta: Option<MoveMeta>,
    pub name: String,
    pub names: Vec<Name>,
    pub past_values: Vec<PastValue>,
    pub power: Option<u16>,
    pub pp: Option<u8>,
    pub priority: i8,
    pub stat_changes: Vec<StatChange>,
    pub super_contest_effect: Option<SuperContestEffectReference>,
    pub target: MoveTargetReference,
    #[serde(rename = "type")]
    pub pokemon_type: TypeReference,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ContestCombos {
    pub normal: Option<ContestCombo>,
    #[serde(rename = "super")]
    pub super_contest: Option<ContestCombo>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ContestCombo {
    pub use_after: Option<Vec<MoveReference>>,
    pub use_before: Option<Vec<MoveReference>>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MoveReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ContestEffectReference {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ContestTypeReference {
    pub name: ContestType,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DamageClassReference {
    pub name: DamageClass,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EffectChange {
    pub effect_entries: Vec<EffectEntry>,
    pub version_group: VersionGroupReference,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: LanguageReference,
    pub version_group: VersionGroupReference,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PokemonReference {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MachineEntry {
    pub machine: MachineReference,
    pub version_group: VersionGroupReference,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MachineReference {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MoveMeta {
    pub ailment: AilmentReference,
    pub ailment_chance: u8,
    pub category: CategoryReference,
    pub crit_rate: u8,
    pub drain: i8,
    pub flinch_chance: u8,
    pub healing: i8,
    pub max_hits: Option<u8>,
    pub max_turns: Option<u8>,
    pub min_hits: Option<u8>,
    pub min_turns: Option<u8>,
    pub stat_chance: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AilmentReference {
    pub name: Ailment,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CategoryReference {
    pub name: Category,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PastValue {
    pub accuracy: Option<u32>,
    pub effect_chance: Option<u32>,
    pub effect_entries: Vec<EffectEntry>,
    pub power: Option<u32>,
    pub pp: Option<u32>,
    #[serde(rename = "type")]
    pub past_type: Option<TypeReference>,
    pub version_group: VersionGroupReference,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StatChange {
    pub change: i8,
    pub stat: StatReference,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StatReference {
    pub name: Stat,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SuperContestEffectReference {
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MoveTargetReference {
    pub name: MoveTarget,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TypeReference {
    pub name: PokemonType,
    pub url: String,
}

// Enum definitions for type-safe reference values
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DamageClass {
    Physical,
    Special,
    Status,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Ailment {
    Burn,
    Confusion,
    Disable,
    Embargo,
    Freeze,
    HealBlock,
    Infatuation,
    Ingrain,
    LeechSeed,
    Nightmare,
    NoTypeImmunity,
    None,
    Paralysis,
    PerishSong,
    Poison,
    Silence,
    Sleep,
    TarShot,
    Torment,
    Trap,
    Unknown,
    Yawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Ailment,
    Damage,
    #[serde(rename = "damage+ailment")]
    DamageAilment,
    #[serde(rename = "damage+heal")]
    DamageHeal,
    #[serde(rename = "damage+lower")]
    DamageLower,
    #[serde(rename = "damage+raise")]
    DamageRaise,
    FieldEffect,
    ForceSwitch,
    Heal,
    NetGoodStats,
    #[serde(rename = "ohko")]
    OHKO,
    Swagger,
    Unique,
    WholeFieldEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Stat {
    Accuracy,
    Attack,
    Defense,
    Evasion,
    SpecialAttack,
    SpecialDefense,
    Speed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MoveTarget {
    AllAllies,
    AllOpponents,
    AllOtherPokemon,
    AllPokemon,
    Ally,
    EntireField,
    FaintingPokemon,
    OpponentsField,
    RandomOpponent,
    SelectedPokemon,
    SelectedPokemonMeFirst,
    SpecificMove,
    User,
    UserAndAllies,
    UserOrAlly,
    UsersField,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PokemonType {
    Bug,
    Dark,
    Dragon,
    Electric,
    Fairy,
    Fighting,
    Fire,
    Flying,
    Ghost,
    Grass,
    Ground,
    Ice,
    Normal,
    Poison,
    Psychic,
    Rock,
    Shadow,
    Stellar,
    Steel,
    Unknown,
    Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ContestType {
    Beauty,
    Cool,
    Cute,
    Smart,
    Tough,
}
