pub mod board;
pub mod q_learning;

use board::BitBoard;
use q_learning::QLearningAgent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameState {
  board: BitBoard,
  current_player: bool,
}

#[wasm_bindgen]
impl GameState {
  pub fn new(second_player: bool) -> Self {
    Self {
      board: BitBoard::new(),
      current_player: second_player,
    }
  }

  pub fn make_agent(file_path: &str) {
    QLearningAgent::load_from_file(file_path);
  }

  pub fn drop_disc(&mut self, col: usize, player: bool) {
    let _ = self.board.drop_disc(col, player);
  }

  pub fn get_board(&self, player: bool) -> u64 {
    self.board.get_player(player)
  }

  pub fn turn_change(&mut self) -> bool {
    self.current_player = !self.current_player;
    self.current_player
  }

  pub fn get_reward(&self, player: bool) -> f64 {
    match self.board.judge() {
      Some(0) => 0.0,
      Some(1) => {
        if player {
          1.0
        } else {
          -1.0
        }
      }
      Some(2) => {
        if player {
          -1.0
        } else {
          1.0
        }
      }
      _ => 0.0,
    }
  }

  pub fn judge(&self) -> Option<i32> {
    self.board.judge()
  }

  pub fn reset(&mut self) {
    self.board = BitBoard::new();
    self.current_player = true;
  }
}
