pub mod commands;
mod enums;
pub mod tactics;
mod words;

use crate::words::Canditates;
use once_cell::sync::Lazy;

pub static CANDITATES: Lazy<Canditates> = Lazy::new(|| Canditates {});
