
use std::io;
use std::str::FromStr;
use crate::characters::{
    character_traits::Character, 
    character_type::CharacterType, 
    character_base::CharacterBase, 
    Knight, Archer, Thief, Wizard
};
use crate::inventory::character_inventory::CharacterInventory;

pub fn read_input<T: FromStr>() -> T {
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

pub fn init_character_stats() -> CharacterBase {
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

pub fn select_character_type() -> CharacterType {
    loop {
        println!("Select your character type:");
        println!("1. Knight");
        println!("2. Archer");
        println!("3. Wizard");
        println!("4. Thief");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => return CharacterType::Knight,
            "2" => return CharacterType::Archer,
            "3" => return CharacterType::Wizard,
            "4" => return CharacterType::Thief,
            _ => println!("Invalid choice, please try again."),
        }
    }
}

pub fn create_character(
    character_type: CharacterType, 
    stats: CharacterBase, 
    inventory: CharacterInventory
) -> Box<dyn Character> {
    match character_type {
        CharacterType::Knight => Box::new(Knight::new(stats, inventory)),
        CharacterType::Archer => Box::new(Archer::new(stats, inventory)),
        CharacterType::Wizard => Box::new(Wizard::new(stats, inventory)),
        CharacterType::Thief => Box::new(Thief::new(stats, inventory)),
    }
}
