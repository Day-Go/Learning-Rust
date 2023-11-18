use crate::equipment::armor::Armor;
use crate::equipment::weapon::Weapon;


pub enum EquipmentType {
    Weapon(Weapon),
    Armor(Armor)
}
