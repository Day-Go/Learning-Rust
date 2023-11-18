use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;

pub trait Character {
    fn new(stats: CharacterBase) -> Self;
}

pub trait Combat {
    fn attack(&self) -> u32;
    fn defend(&self) -> u32;
    fn special(&self) -> u32;
}

pub trait Inventory {
    fn add_item(&mut self, item: EquipmentType) -> ();
    fn remove_item(&mut self, item: EquipmentType) -> ();
}