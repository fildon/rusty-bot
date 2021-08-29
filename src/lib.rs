use wasm_bindgen::prelude::*;
use web_sys::console;

mod bitboard;

#[wasm_bindgen]
pub fn hello() {
    console::log_1(&"Hello from rusty-bot".into());
    bitboard::create_new_board();
}
