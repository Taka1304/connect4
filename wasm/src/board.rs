use wasm_bindgen::prelude::*;

const ROWS: usize = 6;
const COLS: usize = 7;

// ボードの状態を保持するための構造体
#[wasm_bindgen]
#[derive(Clone)]
pub struct BitBoard {
  pub player1: u64, // プレイヤー1のビットボード
  pub player2: u64, // プレイヤー2のビットボード
}

impl BitBoard {
  pub fn new() -> Self {
    BitBoard {
      player1: 0,
      player2: 0,
    }
  }

  // 列が埋まっているかを確認する関数
  pub fn is_column_full(&self, col: usize) -> bool {
    let combined_board = self.player1 | self.player2;
    combined_board & (1 << (col * ROWS + (ROWS - 1))) != 0
  }

  // ビットボードの値を更新する関数
  pub fn drop_disc(&mut self, col: usize, player: bool) -> Result<(), &'static str> {
    if self.is_column_full(col) {
      return Err("Column is full");
    }

    let combined_board = self.player1 | self.player2;
    let mut bit_position = col * ROWS;
    while combined_board & (1 << bit_position) != 0 {
      bit_position += 1;
    }

    if player {
      self.player1 |= 1 << bit_position;
    } else {
      self.player2 |= 1 << bit_position;
    }

    Ok(())
  }

  // ゲームが終了したかを判定する関数
  pub fn judge(&self) -> Option<i32> {
    let directions = [
      1,                   // 水平方向
      ROWS as isize,       // 垂直方向
      (ROWS + 1) as isize, // 斜め右下
      (ROWS - 1) as isize, // 斜め左下
    ];

    for &player_board in [self.player1, self.player2].iter() {
      for &dir in &directions {
        let mut bb = player_board;
        for s in 0..=3 {
          bb &= player_board >> (dir * s);
        }
        if bb != 0 {
          return Some(if player_board == self.player1 { 1 } else { 2 });
        }
      }
    }

    let combined_board = self.player1 | self.player2;
    if combined_board.count_ones() as usize == ROWS * COLS {
      return Some(0); // 引き分け
    }

    None
  }
}
