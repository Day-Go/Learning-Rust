use crate::traits::{Attack, Defend, SpecialMove};

pub struct Thief {
    pub strength: u32,
    pub vitality: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub luck :u32
}

impl Attack for Thief {
    fn attack(&self) -> u32 {
        self.luck
    }
}

impl Defend for Thief {
    fn defend(&self) -> u32 {
        self.vitality
    }
}

impl SpecialMove for Thief {
    fn special(&self) -> u32 {
        self.luck + self.agility
    }
}

