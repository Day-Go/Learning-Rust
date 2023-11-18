use crate::equipment::equipment_type::EquipmentType;
use crate::inventory::character_inventory::CharacterInventory;

pub enum CharacterType {
    Knight,
    Archer,
    Wizard,
    Thief,
}

pub struct CharacterBase {
    pub strength: u32,
    pub vitality: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub luck: u32,
}

impl CharacterBase {
    pub fn hp(&self) -> u32 {
        self.vitality * 10 + self.strength * 2
    }

    pub fn mp(&self) -> u32 {
        self.intelligence * 10 + self.agility * 2
    }
}

pub trait Character {
    fn new(stats: CharacterBase, inventory: CharacterInventory) -> Self where Self: Sized;
    fn attack(&self) -> u32;
    fn defend(&self) -> u32;
    fn special(&self) -> u32;

    fn get_inventory(&mut self) -> &mut CharacterInventory;
    fn get_inventory_length(&self) -> usize;

    fn add_item_to_inventory(&mut self, item: EquipmentType) {
        self.get_inventory().add_item(item);
    }

    fn remove_item_from_inventory(&mut self, item: EquipmentType) {
        self.get_inventory().remove_item(item);
    }

    fn inventory_length(&self) -> usize {
        self.get_inventory_length()
    }
}
