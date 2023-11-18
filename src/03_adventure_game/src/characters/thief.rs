use crate::characters::character_traits::{
    Character, Combat
};
use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;
use crate::inventory::character_inventory::CharacterInventory;


pub struct Thief {
    pub stats: CharacterBase,
    pub inventory: CharacterInventory
}

impl Character for Thief {
    fn new(stats: CharacterBase, inventory: CharacterInventory) -> Self {
        Thief { 
            stats, 
            inventory
        }
    }
}

impl Combat for Thief {
    fn attack(&self) -> u32 {
        self.stats.luck
    }

    fn defend(&self) -> u32 {
        self.stats.vitality
    }

    fn special(&self) -> u32 {
        self.stats.luck + self.stats.agility
    }
}

