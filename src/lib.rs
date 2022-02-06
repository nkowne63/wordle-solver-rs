#[cfg(feature = "local")]
mod commands;
mod enums;
mod leko_competition;
pub mod repls;
mod tactics;
mod words;

use crate::words::Canditates;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub static CANDITATES: Lazy<Canditates> = Lazy::new(|| Canditates {});

#[allow(clippy::mutex_atomic)]
static SHOW_CONSOLE: Lazy<Arc<Mutex<bool>>> = Lazy::new(|| Arc::new(Mutex::new(false)));
pub fn set_show_console(show: bool) {
    *SHOW_CONSOLE.lock().unwrap() = show;
}
pub fn get_show_console() -> bool {
    *SHOW_CONSOLE.lock().unwrap()
}
