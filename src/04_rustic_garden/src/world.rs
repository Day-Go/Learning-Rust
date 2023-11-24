use rand::Rng;
use crate::cell::{Cell, Water, Lava};


pub struct World {
    grid: Vec<Vec<Box<dyn Cell>>>,
    width: u32,
    height: u32
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
       let mut grid = Vec::new();

        for _ in 0..height {
            let mut row = Vec::new();

            for _ in 0..width {
                if rand::thread_rng().gen_bool(0.5) {
                    row.push(Box::new(Water {
                        temperature: rand::thread_rng().gen_range(-10..110) 
                    }) as Box<dyn Cell>);
                } else {
                    row.push(Box::new(Lava {
                        temperature: rand::thread_rng().gen_range(-650..1400) 
                    }) as Box<dyn Cell>);
                }
            }
            grid.push(row);
        }

        World { grid, width, height }
    }
}
