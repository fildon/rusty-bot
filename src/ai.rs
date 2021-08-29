use crate::bitboard;

fn heuristic_evaluation(state: &bitboard::GameState) -> f64 {
  0.0 // TODO
}

fn minimax(state: &bitboard::GameState, depth: &u8, min: &f64, max: &f64) -> f64 {
  match state.leaf_value {
    bitboard::LeafValue::Win => f64::INFINITY,
    bitboard::LeafValue::Loss => f64::NEG_INFINITY,
    bitboard::LeafValue::Draw => 0.0,
    _ => 0.0, // TODO we need to skip this one
  }; // TODO this doesn't return?

  if depth < &1 {
    return heuristic_evaluation(state);
  }

  0.0 // TODO
}

pub fn pick_best_move(bitboard1: u64, bitboard2: u64) -> usize {
  // TODO parse u64 bitboard to a state
  let state = bitboard::create_new_board();

  let legal_moves = bitboard::get_legal_moves(state.height);

  let mut best_value = f64::NEG_INFINITY;
  let mut best_move: usize = 0;
  for current_move in legal_moves.iter() {
    let mut current_value = minimax(
      &bitboard::play_move(&state, *current_move),
      &6,
      &f64::NEG_INFINITY,
      &f64::INFINITY,
    );
    // invert for player 2
    if !state.to_play {
      current_value = -current_value;
    }
    if current_value > best_value {
      best_value = current_value;
      best_move = *current_move;
    }
  }

  return best_move;
}
