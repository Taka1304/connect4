use std::time::Instant;
use wasm::{q_learning::QLearningAgent, GameState};

fn main() {
  let r_file_path = "red_q_table.json";
  let y_file_path = "yellow_q_table.json";
  let episodes = 10000;
  let mut game = GameState::new(false);

  let mut red = QLearningAgent::load_from_file(r_file_path);
  let mut yellow = QLearningAgent::load_from_file(y_file_path);
  let mut r_board = game.get_board(true);
  let mut y_board = game.get_board(false);

  let start_time = Instant::now();

  for _ in 0..episodes {
    while game.judge().is_none() {
      let player = game.turn_change();
      let action: usize;

      if player {
        action = red.choose_action(&game);
        game.drop_disc(action, player);
      } else {
        action = yellow.choose_action(&game);
        game.drop_disc(action, player);
      }

      let next_r = game.get_board(true);
      let next_y = game.get_board(false);

      let reward = game.get_reward(player);
      red.update_q_values(&(r_board, y_board), action, reward, &(next_r, next_y));
      yellow.update_q_values(&(y_board, r_board), action, reward, &(next_y, next_r));

      r_board = next_r;
      y_board = next_y;
    }

    game.reset();
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
