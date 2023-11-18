pub trait Attack {
    fn attack(&self) -> u32;
}

pub trait Defend {
    fn defend(&self) -> u32;
}

pub trait SpecialMove {
    fn special(&self) -> u32;
}