use crate::bitboard;
use web_sys::console;

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
  let mut legal_moves = bitboard::get_legal_moves(state.height);

  legal_moves.sort_by(|a, b| {
    let a_state = bitboard::play_move(&state, *a);
    let a_value = heuristic_evaluation(a_state.bitboard[0], a_state.bitboard[1]);

    let b_state = bitboard::play_move(&state, *b);
    let b_value = heuristic_evaluation(b_state.bitboard[0], b_state.bitboard[1]);

    if state.to_play {
      b_value.cmp(&a_value)
    } else {
      a_value.cmp(&b_value)
    }
  });

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

pub fn pick_best_move(state: &bitboard::GameState, depth: u8, debug: bool) -> usize {
  if debug {
    console::log_1(&"pick_best_move start".into());
    console::log_1(&format!("{:?}", state).into());
    console::log_1(&format!("depth: {}", depth).into());
  }

  let mut legal_moves = bitboard::get_legal_moves(state.height);

  if debug {
    console::log_1(
      &format!(
        "legal_moves: {}",
        &legal_moves
          .iter()
          .map(|&v| v.to_string())
          .collect::<Vec<String>>()
          .join(", ")
      )
      .into(),
    );
  }

  legal_moves.sort_by(|a, b| {
    let a_state = bitboard::play_move(&state, *a);
    let a_value = heuristic_evaluation(a_state.bitboard[0], a_state.bitboard[1]);

    let b_state = bitboard::play_move(&state, *b);
    let b_value = heuristic_evaluation(b_state.bitboard[0], b_state.bitboard[1]);

    if state.to_play {
      b_value.cmp(&a_value)
    } else {
      a_value.cmp(&b_value)
    }
  });

  let mut best_value = f64::NEG_INFINITY;
  let mut best_move: usize = legal_moves[0];
  for current_move in legal_moves.iter() {
    let mut current_value = minimax(
      bitboard::play_move(&state, *current_move),
      depth,
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

  if debug {
    console::log_1(&format!("best_move: {}", best_move).into());
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
    let opening_move = pick_best_move(&blank_board, 9, false);
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

  //#[test] // Test no longer needed. Used as part of a bug investigation.
  fn _illegal_move_regression() {
    let error_board = create_game_state(5682038189, 13205502089746);
    let error_move = pick_best_move(&error_board, 10, false);
    assert_eq!(error_move, 5)
  }
}
