use crate::characters::character_traits::{
    Character, Combat
};
use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;
use crate::inventory::character_inventory::CharacterInventory;

pub struct Knight {
    pub stats: CharacterBase,
    pub inventory: CharacterInventory
}

impl Character for Knight {
    fn new(stats: CharacterBase, inventory: CharacterInventory) -> Self {
        Knight { 
            stats, 
            inventory
        }
    }
}

impl Combat for Knight {
    fn attack(&self) -> u32 {
        self.stats.strength
    }

    fn defend(&self) -> u32 {
        self.stats.vitality
    }

    fn special(&self) -> u32 {
        self.stats.strength + self.stats.vitality
    }
}