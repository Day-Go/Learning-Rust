use crate::traits::Combat;
use crate::equipment::equipment_type::EquipmentType;

pub struct Knight {
    pub strength: u32,
    pub vitality: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub luck :u32,
    pub equipment: Vec<EquipmentType>
}

impl Combat for Knight {
    fn attack(&self) -> u32 {
        self.strength
    }

    fn defend(&self) -> u32 {
        self.vitality
    }

    fn special(&self) -> u32 {
        self.strength + self.vitality
    }
}
