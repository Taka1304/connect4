mod q_learning;

use q_learning::QLearningAgent;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

const ROWS: usize = 6;
const COLS: usize = 7;

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Player {
  Empty,
  Red,
  Yellow,
}

#[wasm_bindgen]
pub struct GameState {
  board: [[Player; COLS]; ROWS],
  current_player: Player,
}

#[wasm_bindgen]
impl GameState {
  pub fn new(first: Player) -> Self {
    Self {
      board: [[Player::Empty; COLS]; ROWS],
      current_player: first,
    }
  }

  pub fn drop_disc(&mut self, col: usize) -> bool {
    if col >= COLS {
      return false;
    }
    for row in (0..ROWS).rev() {
      if let Player::Empty = self.board[row][col] {
        self.board[row][col] = self.current_player;
        self.current_player = match self.current_player {
          Player::Red => Player::Yellow,
          Player::Yellow => Player::Red,
          Player::Empty => Player::Empty,
        };
        return true;
      }
    }
    false
  }

  pub fn get_board(&self) -> Vec<u8> {
    self
      .board
      .iter()
      .flatten()
      .map(|&p| match p {
        Player::Empty => 0,
        Player::Red => 1,
        Player::Yellow => 2,
      })
      .collect()
  }

  pub fn current_player(&self) -> u8 {
    match self.current_player {
      Player::Empty => 0,
      Player::Red => 1,
      Player::Yellow => 2,
    }
  }

  pub fn ai_move(&mut self) -> usize {
    let state = self.get_state();
    let mut agent = QLearningAgent::new();
    let action = agent.choose_action(&state);
    self.drop_disc(action);
    action
  }

  fn get_state(&self) -> String {
    serde_json::to_string(&self.board).unwrap()
  }

  pub fn is_game_over(&self) -> Option<Player> {
    // Check horizontal
    for row in 0..ROWS {
      for col in 0..COLS - 3 {
        if self.board[row][col] != Player::Empty
          && self.board[row][col] == self.board[row][col + 1]
          && self.board[row][col] == self.board[row][col + 2]
          && self.board[row][col] == self.board[row][col + 3]
        {
          return Some(self.board[row][col]);
        }
      }
    }

    // Check vertical
    for col in 0..COLS {
      for row in 0..ROWS - 3 {
        if self.board[row][col] != Player::Empty
          && self.board[row][col] == self.board[row + 1][col]
          && self.board[row][col] == self.board[row + 2][col]
          && self.board[row][col] == self.board[row + 3][col]
        {
          return Some(self.board[row][col]);
        }
      }
    }

    // Check diagonal (bottom-left to top-right)
    for row in 0..ROWS - 3 {
      for col in 0..COLS - 3 {
        if self.board[row][col] != Player::Empty
          && self.board[row][col] == self.board[row + 1][col + 1]
          && self.board[row][col] == self.board[row + 2][col + 2]
          && self.board[row][col] == self.board[row + 3][col + 3]
        {
          return Some(self.board[row][col]);
        }
      }
    }

    // Check diagonal (top-left to bottom-right)
    for row in 3..ROWS {
      for col in 0..COLS - 3 {
        if self.board[row][col] != Player::Empty
          && self.board[row][col] == self.board[row - 1][col + 1]
          && self.board[row][col] == self.board[row - 2][col + 2]
          && self.board[row][col] == self.board[row - 3][col + 3]
        {
          return Some(self.board[row][col]);
        }
      }
    }

    // Check for draw
    if self
      .board
      .iter()
      .all(|row| row.iter().all(|&cell| cell != Player::Empty))
    {
      return Some(Player::Empty); // Representing a draw
    }

    None
  }

  fn get_reward(&self) -> f64 {
    if let Some(winner) = self.is_game_over() {
      match winner {
        Player::Red => 1.0,
        Player::Yellow => -1.0,
        Player::Empty => 0.01, // Draw
      }
    } else {
      0.0
    }
  }

  pub fn train(&mut self, episodes: usize) {
    let mut agent = QLearningAgent::new();
    for _ in 0..episodes {
      let mut state = self.get_state();
      while self.is_game_over().is_none() {
        let action = agent.choose_action(&state);
        self.drop_disc(action);
        let next_state = self.get_state();
        let reward = self.get_reward();
        agent.update_q_values(&state, action, reward, &next_state);
        state = next_state;
      }
      self.reset();
    }
  }

  fn reset(&mut self) {
    self.board = [[Player::Empty; COLS]; ROWS];
    self.current_player = Player::Red;
  }
}
