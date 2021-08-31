use wasm_bindgen::prelude::*;

mod ai;
mod bitboard;

#[wasm_bindgen]
pub fn pick_best_move(bitboard1: u64, bitboard2: u64, depth: u8, debug: bool) -> usize {
  ai::pick_best_move(
    bitboard::create_game_state(bitboard1, bitboard2),
    depth,
    debug,
  )
}
