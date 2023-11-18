use crate::traits::Combat;
use crate::equipment::equipment_type::EquipmentType;
use crate::characters::character_base::CharacterBase;


pub struct Archer {
    pub stats: CharacterBase,
    pub equipment: Vec<EquipmentType>
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

