use std::time::Instant;
use wasm::{board::BitBoard, q_learning::QLearningAgent};

fn main() {
  let r_file_path = "red_q_table.json";
  let y_file_path = "yellow_q_table.json";
  let episodes = 10000;

  let mut board = BitBoard::new();
  let mut turn = false;
  let mut red = QLearningAgent::load_from_file(r_file_path);
  let mut yellow = QLearningAgent::load_from_file(y_file_path);

  let start_time = Instant::now();

  for _ in 0..episodes {
    while board.judge().is_none() {
      turn = !turn;
      let action: usize;

      if turn {
        action = red.choose_action(&board);
        let _ = board.drop_disc(action, turn);
      } else {
        action = yellow.choose_action(&board);
        let _ = board.drop_disc(action, turn);
      }

      let next_board = board.clone();

      red.update_q_values(
        &(board.player1, board.player2),
        action,
        red.clone().get_reward(&board, turn),
        &(next_board.player1, next_board.player2),
      );
      yellow.update_q_values(
        &(board.player2, board.player1),
        action,
        yellow.clone().get_reward(&board, turn),
        &(next_board.player2, next_board.player1),
      );

      board = next_board;
    }

    board = BitBoard::new();
  }

  let elapsed_time = start_time.elapsed();

  println!("Elapsed time: {:?}", elapsed_time);
  red.save_to_file(r_file_path);
  yellow.save_to_file(y_file_path);

  println!(
    "Trained Q-learning agents and saved to {} and {}",
    r_file_path, y_file_path
  );
}
