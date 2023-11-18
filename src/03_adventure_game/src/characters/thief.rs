use crate::traits::Combat;
use crate::equipment::equipment_type::EquipmentType;

pub struct Thief {
    pub strength: u32,
    pub vitality: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub luck :u32,
    pub equipment: Vec<EquipmentType>
}

impl Combat for Thief {
    fn attack(&self) -> u32 {
        self.luck
    }

    fn defend(&self) -> u32 {
        self.vitality
    }

    fn special(&self) -> u32 {
        self.luck + self.agility
    }
}

