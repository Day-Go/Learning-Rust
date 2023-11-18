use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;
use crate::inventory::character_inventory::CharacterInventory;


pub trait Character {
    fn new(stats: CharacterBase, inventory: CharacterInventory) -> Self;
}

pub trait Combat {
    fn attack(&self) -> u32;
    fn defend(&self) -> u32;
    fn special(&self) -> u32;
}