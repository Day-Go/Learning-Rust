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