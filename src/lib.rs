#![feature(concat_idents)]

mod board;
#[macro_use]
mod board4x4;
#[macro_use]
mod board9x9;
pub mod utils;

pub use board::*;
pub use board4x4::*;
pub use board9x9::*;
