pub enum LeafValue {
  Win,
  Loss,
  Draw,
  Unknown,
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
  bitboard: [u64; 2],
  pub height: [u8; 7],
  pub to_play: bool,
  pub leaf_value: LeafValue,
}

pub fn create_new_board() -> GameState {
  GameState {
    bitboard: [0, 0],
    height: [0, 7, 14, 21, 28, 35, 42],
    to_play: true,
    leaf_value: LeafValue::Unknown,
  }
}

fn is_draw(height: &[u8; 7]) -> bool {
  // draw if height is topped out in every column
  height == &[6, 13, 20, 27, 34, 41, 48]
}

const DIRECTIONS: [u8; 4] = [1, 7, 6, 8];
fn is_win(bitboard: &u64) -> bool {
  DIRECTIONS.iter().any(|&direction| {
    let bb = bitboard & (bitboard >> direction);
    (bb & (bb >> (2 * direction))) != 0
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
      LeafValue::Unknown
    },
  }
}

pub fn get_legal_moves(height: [u8; 7]) -> Vec<usize> {
  (0..=6)
    .into_iter()
    // TODO scope to improve this filter
    .filter(|&col| {
      ![6, 13, 20, 27, 34, 41, 48]
        .iter()
        .any(|&top| top == 1 << height[col])
    })
    .collect()
}
