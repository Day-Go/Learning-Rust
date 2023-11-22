use crate::cell::{Lava, Water};

mod cell;
mod world;

fn main() {
    let cell: Lava = Lava { temperature: 84 };
    println!("Lava temperature is {}", cell.temperature);
    println!("The lava is {}", cell.state());
}
