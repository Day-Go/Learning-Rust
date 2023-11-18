mod game;
mod characters;
mod equipment;
mod inventory;

use crate::characters::character_base::CharacterBase;
use crate::characters::{Knight, Archer, Thief, Wizard};
use crate::inventory::character_inventory::CharacterInventory;
use crate::equipment::equipment_type::EquipmentType;
use crate::equipment::weapon::{Weapon, WeaponType};
use crate::game::helper::{
    read_input, 
    init_character_stats, 
    select_character_type,
    create_character
};



fn main() {
    let character_type = game::helper::select_character_type();
    let stats: CharacterBase = game::helper::init_character_stats();
    let inventory = CharacterInventory::new();

    let mut character = game::helper::create_character(
        character_type,
        stats,
        inventory
    );


    println!("Knight attack power: {}", character.attack());

    let sword = Weapon {
        weapon_type: WeaponType::Sword,
        attack: 5,
        attack_speed: 1,
    };

    let sword_equipment = EquipmentType::Weapon(sword);

    let result = character.add_item_to_inventory(sword_equipment);
    println!("Knight found a new piece of equipment: {}", 
              character.get_inventory_length());
}