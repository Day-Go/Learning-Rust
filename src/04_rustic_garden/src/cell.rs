use std::fmt::{Display, Result, Formatter};

enum State {
    Solid,
    Liquid,
    Gas
}

pub trait Cell {
    fn update(&self);
}

pub struct Water {
    pub temperature: i32
}

pub struct Lava {
    pub temperature: i32
}

impl Water {
    pub fn state(&self) -> State {
        if self.temperature < 0 {
            State::Solid
        }
        else if self.temperature > 100
        {
            State::Gas
        }
        else {
            State:: Liquid
        }
    }
}

impl Lava {
    pub fn state(&self) -> State {
        if self.temperature < 700 {
            State::Solid
        }
        else {
            State::Liquid 
        }
    }

}

impl Display for Lava {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        todo!()
    }
}
