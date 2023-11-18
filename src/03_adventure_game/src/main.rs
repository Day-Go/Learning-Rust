mod traits;
mod characters;
mod equipment;

use crate::characters::character_base::CharacterBase;
use crate::characters::knight::Knight;
use crate::traits::{Combat, Inventory};
use crate::equipment::equipment_type::EquipmentType;
use crate::equipment::weapon::{Weapon, WeaponType};



fn main() {
    let mut knight = Knight { 
        stats: CharacterBase{
            strength: 10, vitality: 15, agility: 8, 
            intelligence: 4, luck: 5
        }, 
        equipment:  Vec::new() 
    };
    println!("Knight attack power: {}", knight.attack());

    let sword = Weapon {
        weapon_type: WeaponType::Sword,
        attack: 5,
        attack_speed: 1,
    };

    let sword_equipment = EquipmentType::Weapon(sword);

    let result = knight.add_item(sword_equipment);
    println!("Knight found a new piece of equipment: {}", knight.equipment.len());
}