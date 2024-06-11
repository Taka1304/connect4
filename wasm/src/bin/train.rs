use std::time::Instant;
use wasm::{q_learning::QLearningAgent, GameState};

fn main() {
  let r_file_path = "red_q_table.json";
  let y_file_path = "yellow_q_table.json";
  let episodes = 10000;
  let mut game = GameState::new(false);

  let mut red = QLearningAgent::load_from_file(r_file_path);
  let mut yellow = QLearningAgent::load_from_file(y_file_path);
  let mut r_state = (game.get_board(true), game.get_board(false));
  let mut y_state = (game.get_board(false), game.get_board(true));

  let start_time = Instant::now();

  for _ in 0..episodes {
    while game.judge().is_none() {
      let player = game.turn_change();

      if player {
        let action = red.choose_action(&game);
        game.drop_disc(action, player);
        let next_state = (game.get_board(true), game.get_board(false));
        let reward = game.get_reward(player);
        red.update_q_values(&r_state, action, reward, &next_state);
        r_state = next_state;
      } else {
        let action = yellow.choose_action(&game);
        game.drop_disc(action, player);
        let next_state = (game.get_board(false), game.get_board(true));
        let reward = game.get_reward(player);
        yellow.update_q_values(&y_state, action, reward, &next_state);
        y_state = next_state;
      }
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
