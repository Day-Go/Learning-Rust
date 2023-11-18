pub trait Combat {
    fn attack(&self) -> u32;
    fn defend(&self) -> u32;
    fn special(&self) -> u32;
}