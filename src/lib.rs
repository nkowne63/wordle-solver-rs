pub mod commands;
mod enums;
pub mod tactics;
mod words;

use crate::words::Canditates;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub static CANDITATES: Lazy<Canditates> = Lazy::new(|| Canditates {});

#[allow(clippy::mutex_atomic)]
pub static SHOW_CONSOLE: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));
