pub enum WeaponType {
    Sword,
    Mace,
    Staff,
    Wand,
    Bow,
    Crossbow,
    Dagger,
}

pub struct Weapon {
    pub weapon_type: WeaponType,
    pub attack: u32,
    pub attack_speed: u32,
}
