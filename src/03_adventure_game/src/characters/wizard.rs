use crate::characters::character_traits::Character;
use crate::characters::character_base::CharacterBase;
use crate::inventory::character_inventory::CharacterInventory;


pub struct Wizard {
    pub stats: CharacterBase,
    pub inventory: CharacterInventory
}

impl Character for Wizard {
    fn new(stats: CharacterBase, inventory: CharacterInventory) -> Self {
        Wizard {stats, inventory}
    }

    fn attack(&self) -> u32 {
        self.stats.intelligence
    }

    fn defend(&self) -> u32 {
        self.stats.vitality
    }

    fn special(&self) -> u32 {
        self.stats.intelligence + self.stats.vitality
    }

    fn get_inventory(&mut self) -> &mut CharacterInventory {
        &mut self.inventory
    }

    fn get_inventory_length(&self) -> usize {
        self.inventory.equipment.len()
    }
}


