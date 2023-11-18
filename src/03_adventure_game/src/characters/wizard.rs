use crate::traits::{Attack, Defend, SpecialMove};

pub struct Wizard {
    pub strength: u32,
    pub vitality: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub luck :u32    
}

impl Attack for Wizard {
    fn attack(&self) -> u32 {
        self.intelligence
    }
}

impl Defend for Wizard {
    fn defend(&self) -> u32 {
        self.vitality
    }
}

impl SpecialMove for Wizard {
    fn special(&self) -> u32 {
        self.intelligence + self.vitality
    }
}
