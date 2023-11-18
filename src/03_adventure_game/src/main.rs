mod characters;
mod equipment;

use std::io;
use std::str::FromStr;
use crate::characters::character_base::CharacterBase;
use crate::characters::{
    Knight, 
    Archer, 
    Thief, 
    Wizard
};
use crate::characters::character_traits::{
    Character, 
    Combat, 
    Inventory
};
use crate::equipment::equipment_type::EquipmentType;
use crate::equipment::weapon::{Weapon, WeaponType};


fn read_input<T: FromStr>() -> T {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<T>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number"),
        }
    }
}

fn get_character_stats() -> CharacterBase {
    let attributes = ["strength", "vitality", "agility", "intelligence", "luck"];
    let mut values = Vec::new();

    for &attr in &attributes {
        println!("Please enter your character's {}: ", attr);
        values.push(read_input::<u32>());
    }

    CharacterBase {
        strength: values[0],
        vitality: values[1],
        agility: values[2],
        intelligence: values[3],
        luck: values[4],
    }
}


fn create_character<T: Character>() -> T {
    let stats = get_character_stats();

    T::new(stats)
}



fn main() {
    let mut knight: Knight = create_character();

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