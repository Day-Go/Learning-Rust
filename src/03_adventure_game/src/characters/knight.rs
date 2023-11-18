use crate::traits::{Attack, Defend, SpecialMove};

pub struct Knight {
    pub strength: u32,
    pub vitality: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub luck :u32
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
