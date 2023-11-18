pub mod knight;
pub mod archer;
pub mod thief;
pub mod wizard;
pub mod character_traits;

pub use self::{
    knight::Knight,
    archer::Archer,
    thief::Thief,
    wizard::Wizard
};