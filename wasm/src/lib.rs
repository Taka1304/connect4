pub mod board;
pub mod q_learning;

use board::BitBoard;
use q_learning::QLearningAgent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GameState {
  board: BitBoard,
  current_player: bool,
  agent: Option<QLearningAgent>,
}

#[wasm_bindgen]
impl GameState {
  pub fn new(second_player: bool) -> Self {
    Self {
      board: BitBoard::new(),
      current_player: second_player,
      agent: None,
    }
  }

  pub fn load_agent(&mut self, file_path: &str) {
    self.agent = Some(QLearningAgent::load_from_file(file_path));
  }

  pub fn drop_disc(&mut self, col: usize, player: bool) {
    let _ = self.board.drop_disc(col, player);
  }

  pub fn get_board(&self) -> BitBoard {
    self.board.clone()
  }

  pub fn turn_change(&mut self) -> bool {
    self.current_player = !self.current_player;
    self.current_player
  }

  pub fn get_reward(&self, player: bool) -> f64 {
    match self.board.judge() {
      Some(1) if player => 1.0,
      Some(1) => -1.0,
      Some(2) if player => -1.0,
      Some(2) => 1.0,
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

  pub fn choose_action(&mut self) -> usize {
    self.agent.clone().unwrap().choose_action(&self)
  }

  pub fn update_q_values(
    &mut self,
    state: BitBoard,
    action: usize,
    reward: f64,
    next_state: BitBoard,
  ) {
    if let Some(ref mut agent) = self.agent {
      agent.update_q_values(
        &(state.player1, state.player2),
        action,
        reward,
        &(next_state.player1, next_state.player2),
      );
    }
  }
}
