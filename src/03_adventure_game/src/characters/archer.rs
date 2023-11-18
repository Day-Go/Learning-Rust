use crate::traits::{Attack, Defend, SpecialMove};

pub struct Archer {
    pub strength: u32,
    pub vitality: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub luck :u32    
}

impl Attack for Archer {
    fn attack(&self) -> u32 {
        self.agility
    }
}

impl Defend for Archer {
    fn defend(&self) -> u32 {
        self.vitality
    }
}

impl SpecialMove for Archer {
    fn special(&self) -> u32 {
        self.agility + self.intelligence
    }
}
