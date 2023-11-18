use crate::characters::character_traits::{
    Character,
    CharacterBase
};
use crate::inventory::character_inventory::CharacterInventory;


pub struct Thief {
    pub stats: CharacterBase,
    pub inventory: CharacterInventory
}

impl Character for Thief {
    fn new(stats: CharacterBase, inventory: CharacterInventory) -> Self {
        Thief {stats, inventory}
    }

    fn attack(&self) -> u32 {
        self.stats.luck
    }

    fn defend(&self) -> u32 {
        self.stats.vitality
    }

    fn special(&self) -> u32 {
        self.stats.luck + self.stats.agility
    }

    fn get_inventory(&mut self) -> &mut CharacterInventory {
        &mut self.inventory
    }

    fn get_inventory_length(&self) -> usize {
        self.inventory.equipment.len()
    }
}
