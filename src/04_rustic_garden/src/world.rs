use crate::cell::Cell;

pub struct World {
    grid: Vec<Vec<Box<dyn Cell>>>,
    width: u32,
    height: u32
}
