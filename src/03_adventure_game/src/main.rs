mod game;
mod characters;
mod equipment;
mod inventory;

use crate::characters::{
    Knight, 
    Archer, 
    Thief, 
    Wizard
};
use crate::characters::character_traits::Combat;
use crate::equipment::equipment_type::EquipmentType;
use crate::equipment::weapon::{Weapon, WeaponType};


fn main() {
    let mut knight: Knight = game::create_character();

    println!("Knight attack power: {}", knight.attack());

    let sword = Weapon {
        weapon_type: WeaponType::Sword,
        attack: 5,
        attack_speed: 1,
    };

    let sword_equipment = EquipmentType::Weapon(sword);

    let result = knight.inventory.add_item(sword_equipment);
    println!("Knight found a new piece of equipment: {}", 
              knight.inventory.equipment.len());
}