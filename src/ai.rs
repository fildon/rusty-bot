use crate::bitboard;

/**
 * A map of the board with each cell displaying how many wins could pass through it
 * 0s are for the top row which should never come up
 */
const WIN_MAP: [i32; 48] = [
  3, 4, 5, 5, 4, 3, 0, 4, 6, 8, 8, 6, 4, 0, 5, 8, 11, 11, 8, 5, 0, 7, 10, 13, 13, 10, 7, 0, 5, 8,
  11, 11, 8, 5, 0, 4, 6, 8, 8, 6, 4, 0, 3, 4, 5, 5, 4, 3,
];
fn heuristic_evaluation(p1: u64, p2: u64) -> i32 {
  let mut evaluation = 0;

  for location in 0..48 {
    let value = WIN_MAP[location];
    if (p1 >> location) & 1 != 0 {
      evaluation += value
    }
    if (p2 >> location) & 1 != 0 {
      evaluation -= value
    }
  }

  return evaluation;
}

fn minimax(state: bitboard::GameState, depth: u8, min: f64, max: f64) -> f64 {
  if state.leaf_value == bitboard::LeafValue::Win {
    return f64::INFINITY;
  }
  if state.leaf_value == bitboard::LeafValue::Loss {
    return f64::NEG_INFINITY;
  }
  if state.leaf_value == bitboard::LeafValue::Draw {
    return 0.0;
  }

  if depth < 1 {
    return heuristic_evaluation(state.bitboard[0], state.bitboard[1]).into();
  }

  let mut v = if state.to_play {
    min.clone()
  } else {
    max.clone()
  };
  let legal_moves = bitboard::get_legal_moves(state.height);
  for legal_move in legal_moves {
    let child = bitboard::play_move(&state, legal_move);
    let vv = minimax(
      child,
      depth - 1,
      if state.to_play { v } else { min },
      if state.to_play { max } else { v },
    );

    if state.to_play {
      if vv > v {
        v = vv
      }
      if v >= max {
        return max;
      }
    } else {
      if vv < v {
        v = vv
      }
      if v <= min {
        return min;
      }
    }
  }
  return v;
}

pub fn pick_best_move(state: bitboard::GameState) -> usize {
  let legal_moves = bitboard::get_legal_moves(state.height);

  let mut best_value = f64::NEG_INFINITY;
  let mut best_move: usize = 0;
  for current_move in legal_moves.iter() {
    let mut current_value = minimax(
      bitboard::play_move(&state, *current_move),
      9,
      f64::NEG_INFINITY,
      f64::INFINITY,
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::bitboard::create_game_state;
  #[test]

  fn opens_in_middle_column() {
    let blank_board = create_game_state(0, 0);
    let opening_move = pick_best_move(blank_board);
    assert_eq!(opening_move, 3)
  }

  #[test]
  fn test_heuristic_evaluation() {
    assert_eq!(heuristic_evaluation(0, 0), 0);
    assert_eq!(heuristic_evaluation(1, 0), 3);
    assert_eq!(heuristic_evaluation(0, 1), -3);
    assert_eq!(heuristic_evaluation(1, 1), 0);
    assert_eq!(heuristic_evaluation(3, 0), 7);
    assert_eq!(heuristic_evaluation(1 << 21, 0), 7);
  }
}
