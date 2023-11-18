use crate::traits::{Combat, Inventory};
use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;


pub struct Knight {
    pub stats: CharacterBase,
    pub equipment: Vec<EquipmentType>,
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

impl Inventory for Knight {
    fn add_item(&mut self, item: EquipmentType) -> () {
        self.equipment.push(item);
    }

    fn remove_item(&mut self, item: EquipmentType) -> () {
        self.equipment.pop();
    }
}