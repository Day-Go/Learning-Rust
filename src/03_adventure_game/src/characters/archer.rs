use crate::characters::character_traits::{
    Character, Combat
};
use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;
use crate::inventory::character_inventory::CharacterInventory;


pub struct Archer {
    pub stats: CharacterBase,
    pub inventory: CharacterInventory
}

impl Character for Archer {
    fn new(stats: CharacterBase, inventory: CharacterInventory) -> Self {
        Archer { 
            stats, 
            inventory
        }
    }
}

impl Combat for Archer {
    fn attack(&self) -> u32 {
        self.stats.agility
    }

    fn defend(&self) -> u32 {
        self.stats.vitality
    }

    fn special(&self) -> u32 {
        self.stats.agility + self.stats.intelligence
    }
}
