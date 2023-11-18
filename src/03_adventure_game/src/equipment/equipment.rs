use crate::equipment::{Weapon, Armor};

pub enum EquipmentType {
    Weapon(Weapon),
    Armor(Armor)
}
