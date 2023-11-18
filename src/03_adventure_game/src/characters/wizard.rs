use crate::characters::character_traits::{
    Character, Combat
};
use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;
use crate::inventory::character_inventory::CharacterInventory;


pub struct Wizard {
    pub stats: CharacterBase,
    pub inventory: CharacterInventory
}

impl Character for Wizard {
    fn new(stats: CharacterBase, inventory: CharacterInventory) -> Self {
        Wizard { 
            stats, 
            inventory
        }
    }
}

impl Combat for Wizard {
    fn attack(&self) -> u32 {
        self.stats.intelligence
    }

    fn defend(&self) -> u32 {
        self.stats.vitality
    }

    fn special(&self) -> u32 {
        self.stats.intelligence + self.stats.vitality
    }
}
