use crate::characters::character_traits::{
    Character,
    CharacterBase
};
use crate::inventory::character_inventory::CharacterInventory;


pub struct Archer {
    pub stats: CharacterBase,
    pub inventory: CharacterInventory
}

impl Character for Archer {
    fn new(stats: CharacterBase, inventory: CharacterInventory) -> Self {
        Archer { stats, inventory }
    }

    fn attack(&self) -> u32 {
        self.stats.agility
    }

    fn defend(&self) -> u32 {
        self.stats.vitality
    }

    fn special(&self) -> u32 {
        self.stats.agility + self.stats.intelligence
    }

    fn get_inventory(&mut self) -> &mut CharacterInventory {
        &mut self.inventory
    }

    fn get_inventory_length(&self) -> usize {
        self.inventory.equipment.len()
    }
}
