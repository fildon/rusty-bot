#[derive(PartialEq, Eq)]
pub enum LeafValue {
  Win,
  Loss,
  Draw,
  InProgress,
}

pub struct GameState {
  /**
   * ```md
   *   6 13 20 27 34 41 48   55 62     Additional row
   * +---------------------+
   * | 5 12 19 26 33 40 47 | 54 61     top row
   * | 4 11 18 25 32 39 46 | 53 60
   * | 3 10 17 24 31 38 45 | 52 59
   * | 2  9 16 23 30 37 44 | 51 58
   * | 1  8 15 22 29 36 43 | 50 57
   * | 0  7 14 21 28 35 42 | 49 56 63  bottom row
   * +---------------------+
   * ```
   */
  pub bitboard: [u64; 2],
  pub height: [u8; 7],
  pub to_play: bool,
  pub leaf_value: LeafValue,
}

fn count_ones(binary: u64) -> u8 {
  let mut count = 0;
  let mut clone = binary.clone();
  while clone > 0 {
    count = count + 1;
    clone = clone & (clone - 1); // delete lowest 1
  }
  count
}

fn get_height(combinedbitboard: u64) -> [u8; 7] {
  let mut height = [0, 7, 14, 21, 28, 35, 42];

  for col in 0..=6 {
    let mut row = 0;
    while row < 6 && 1 << height[col] & combinedbitboard != 0 {
      height[col] += 1;
      row += 1
    }
  }

  height
}

fn get_leaf_value(bitboard1: u64, bitboard2: u64, height: [u8; 7]) -> LeafValue {
  if is_win(&bitboard1) {
    return LeafValue::Win;
  }
  if is_win(&bitboard2) {
    return LeafValue::Loss;
  }

  if is_draw(&height) {
    return LeafValue::Draw;
  }
  LeafValue::InProgress
}

/**
 * Build a GameState from a pair of bitboards
 *
 * _WARNING_: Does not check that the provided bitboards or the resulting GameState is a legal position
 */
pub fn create_game_state(bitboard1: u64, bitboard2: u64) -> GameState {
  let height = get_height(bitboard1 | bitboard2);
  GameState {
    bitboard: [bitboard1, bitboard2],
    height,
    to_play: count_ones(bitboard1 | bitboard2) % 2 == 0,
    leaf_value: get_leaf_value(bitboard1, bitboard2, height),
  }
}

const TOP: [u8; 7] = [6, 13, 20, 27, 34, 41, 48];
fn is_draw(height: &[u8; 7]) -> bool {
  // draw if height is topped out in every column
  height == &TOP
}

const DIRECTIONS: [u8; 4] = [1, 7, 6, 8];
fn is_win(bitboard: &u64) -> bool {
  DIRECTIONS.iter().any(|&direction| {
    let bb = bitboard & (bitboard >> direction);
    (bb & (bb >> (direction << 1))) != 0
  })
}

pub fn play_move(state: &GameState, column: usize) -> GameState {
  let new_move = 1 << state.height[column];

  let new_board: [u64; 2] = if state.to_play {
    [state.bitboard[0] ^ new_move, state.bitboard[1]]
  } else {
    [state.bitboard[0], state.bitboard[1] ^ new_move]
  };

  let mut new_height = state.height.clone();
  new_height[column] += 1;

  GameState {
    bitboard: new_board,
    height: new_height,
    to_play: !state.to_play,
    leaf_value: if is_win(&new_board[0]) {
      LeafValue::Win
    } else if is_win(&new_board[1]) {
      LeafValue::Loss
    } else if is_draw(&new_height) {
      LeafValue::Draw
    } else {
      LeafValue::InProgress
    },
  }
}

pub fn get_legal_moves(height: [u8; 7]) -> Vec<usize> {
  let mut legal_moves: Vec<usize> = Vec::new();

  for col in 0..=6 {
    // A move is legal iff its column is not topped out
    if TOP[col] != height[col] {
      legal_moves.push(col);
    }
  }

  legal_moves
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn seven_legal_moves() {
    assert_eq!(
      get_legal_moves([0, 7, 14, 21, 28, 35, 42]),
      [0, 1, 2, 3, 4, 5, 6]
    )
  }

  #[test]
  fn zero_legal_moves() {
    assert_eq!(get_legal_moves([6, 13, 20, 27, 34, 41, 48]), [])
  }
}
