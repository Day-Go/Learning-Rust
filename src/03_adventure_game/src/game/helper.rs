
use std::io;
use std::str::FromStr;
use crate::characters::character_traits::Character;
use crate::characters::character_base::CharacterBase;
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

pub fn create_character<T: Character>() -> T {
    let stats = init_character_stats();
    let inventory = CharacterInventory::new();
    T::new(stats, inventory)
}