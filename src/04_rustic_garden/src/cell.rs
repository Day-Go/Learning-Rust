use std::fmt::{Display, Result, Formatter};

pub enum State {
    Solid,
    Liquid,
    Gas
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            State::Solid => write!(f, "Solid"),
            State::Liquid => write!(f, "Liquid"),
            State::Gas => write!(f, "Gas")
        }
    }
}

pub trait Cell {
    fn update(&mut self);
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

impl Cell for Water {
    fn update(&mut self) {
        self.temperature -= 1;
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

impl Cell for Lava {
    fn update(&mut self) {
        self.temperature -= 1;
    }
}   

impl Display for Lava {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        todo!()
    }
}
