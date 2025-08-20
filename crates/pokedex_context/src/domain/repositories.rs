use crate::domain::ability::Ability;
use crate::domain::item::Item;

pub trait IPokedexRepository {
    fn load_all_abilities(&self) -> Vec<Ability>;
    fn load_all_items(&self) -> Vec<Item>;
}
