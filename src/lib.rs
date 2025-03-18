mod debugger;
mod emulator;
mod handlers;
mod keys;
mod runner;
mod ui;

use std::panic;
use wasm_bindgen::prelude::wasm_bindgen;
extern crate console_error_panic_hook;

#[wasm_bindgen]
pub fn start() {
    init_console();
    handlers::set_handlers();
    keys::set_handlers();
    handlers::trigger_select_splash_screen();
}

fn init_console() {
    console_log::init_with_level(log::Level::Debug).expect("Couldn't initialize the console");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}
