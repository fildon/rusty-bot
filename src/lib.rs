use wasm_bindgen::prelude::*;

mod ai;
mod bitboard;

#[wasm_bindgen]
pub fn pick_best_move(bitboard1: u64, bitboard2: u64) {
    ai::pick_best_move(bitboard1, bitboard2);
}
