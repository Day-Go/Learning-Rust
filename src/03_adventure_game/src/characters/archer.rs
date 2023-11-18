use crate::characters::character_traits::{
    Character, Combat, Inventory
};
use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;


pub struct Archer {
    pub stats: CharacterBase,
    pub equipment: Vec<EquipmentType>
}

impl Character for Archer {
    fn new(stats: CharacterBase) -> Self {
        Archer { 
            stats, 
            equipment: Vec::new() 
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

impl Inventory for Archer {
    fn add_item(&mut self, item: EquipmentType) -> () {
        self.equipment.push(item);
    }

    fn remove_item(&mut self, item: EquipmentType) -> () {
        self.equipment.pop();
    }
}