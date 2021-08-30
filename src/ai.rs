use crate::bitboard;

fn heuristic_evaluation(state: &bitboard::GameState) -> f64 {
  0.0 // TODO
}

fn minimax(state: &bitboard::GameState, depth: &u8, min: &f64, max: &f64) -> f64 {
  if state.leaf_value == bitboard::LeafValue::Win {
    return f64::INFINITY;
  }
  if state.leaf_value == bitboard::LeafValue::Loss {
    return f64::NEG_INFINITY;
  }
  if state.leaf_value == bitboard::LeafValue::Draw {
    return 0.0;
  }

  if depth < &1 {
    return heuristic_evaluation(state);
  }

  0.0 // TODO
}

pub fn pick_best_move(state: bitboard::GameState) -> usize {
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
