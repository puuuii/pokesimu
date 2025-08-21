use super::common::{GenerationReference, LanguageReference, Name, RequiredShortEffectEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub is_main_series: bool,
    pub generation: GenerationReference,
    pub names: Vec<Name>,
    pub effect_entries: Vec<RequiredShortEffectEntry>,
}
