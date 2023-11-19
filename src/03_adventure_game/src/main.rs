mod game;
mod characters;
mod equipment;
mod inventory;
mod enemy;

use crate::characters::character_traits::CharacterBase;
use crate::inventory::character_inventory::CharacterInventory;
use crate::equipment::equipment_type::EquipmentType;
use crate::equipment::weapon::{Weapon, WeaponType};
use crate::game::helper::{
    init_character_stats, 
    select_character_type,
    create_character,
    choose_random_enemy_type
};

fn main() {
    let character_type = select_character_type();
    let stats: CharacterBase = init_character_stats();
    let inventory = CharacterInventory::new();

    let mut character = create_character(
        character_type,
        stats,
        inventory
    );

    loop {
        let enemy_type = choose_random_enemy_type();
    }


    // println!("Knight attack power: {}", character.attack());

    // let sword = Weapon {
    //     weapon_type: WeaponType::Sword,
    //     attack: 5,
    //     attack_speed: 1,
    // };

    // let sword_equipment = EquipmentType::Weapon(sword);

    // let result = character.add_item_to_inventory(sword_equipment);
    // println!("Knight found a new piece of equipment: {}", 
    //           character.get_inventory_length());
}