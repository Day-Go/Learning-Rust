use crate::traits::Combat;
use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;


pub struct Thief {
    pub stats: CharacterBase,
    pub equipment: Vec<EquipmentType>
}

impl Combat for Thief {
    fn attack(&self) -> u32 {
        self.stats.luck
    }

    fn defend(&self) -> u32 {
        self.stats.vitality
    }

    fn special(&self) -> u32 {
        self.stats.luck + self.stats.agility
    }
}

