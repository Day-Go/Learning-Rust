use crate::characters::character_traits::{
    Character, Combat, Inventory
};
use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;

pub struct Wizard {
    pub stats: CharacterBase,
    pub equipment: Vec<EquipmentType>
}

impl Character for Wizard {
    fn new(stats: CharacterBase) -> Self {
        Wizard { 
            stats, 
            equipment: Vec::new() 
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
