pub mod board;
pub mod q_learning;

use board::BitBoard;
use q_learning::QLearningAgent;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct VsAi {
  is_first: bool,
  agent: Option<QLearningAgent>,
}

#[wasm_bindgen]
impl VsAi {
  #[wasm_bindgen(constructor)]
  pub fn new(is_first: bool) -> Self {
    if is_first {
      Self {
        is_first,
        agent: Some(QLearningAgent::load_from_file("./red_q_table.json")),
      }
    } else {
      Self {
        is_first,
        agent: Some(QLearningAgent::load_from_file("./yellow_q_table.json")),
      }
    }
  }

  pub fn choose_action(&mut self, board: BitBoard) -> usize {
    // TODO: cloneしまくり
    let _board = board.clone();
    let action = self.agent.clone().unwrap().choose_action(&board);
    let _ = board.clone().drop_disc(action, self.is_first);
    let reward = self
      .agent
      .clone()
      .unwrap()
      .get_reward(&board, self.is_first);
    self.agent.clone().unwrap().update_q_values(
      &(_board.player1, _board.player2),
      action,
      reward,
      &(board.player1, board.player2),
    );
    action
  }
}
