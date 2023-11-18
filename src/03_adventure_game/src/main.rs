mod traits;
mod characters;

use crate::traits::{Attack, Defend, SpecialMove};
use crate::characters::knight::Knight;

fn main() {
    let knight = Knight { 
        strength: 10, vitality: 15, agility: 8, 
        intelligence: 4, luck: 5 };
    println!("Knight attack power: {}", knight.attack());

    // ... similar for other characters
}