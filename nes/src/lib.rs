pub mod error;
pub mod palette;
pub mod consoles;

mod dma;
mod mappers;
mod bus;
mod ppu;
mod controllers;

pub use controllers::JoypadInput;

#[macro_use]
extern crate bitflags;

