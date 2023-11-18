pub enum ArmorCategory {
    Helmet,
    Chest,
    Legs,
    Boots,
    Shield
}

pub struct Armor {
    pub armor_category: ArmorCategory,
    pub strength: u32,
    pub defense: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub luck: u32
}