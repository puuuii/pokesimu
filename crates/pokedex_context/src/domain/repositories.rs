use crate::domain::ability::Ability;

pub trait IPokedexRepository {
    fn load_all_abilities(&self) -> Vec<Ability>;
}
