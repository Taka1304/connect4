use wasm::{q_learning::QLearningAgent, GameState};

fn main() {
  let r_file_path = "red_q_table.json";
  let y_file_path = "yellow_q_table.json";
  let episodes = 10000;
  let mut game = GameState::new(true);

  let mut red = QLearningAgent::load_from_file(r_file_path);
  let mut yellow = QLearningAgent::load_from_file(y_file_path);
  let mut state = (game.get_board(true), game.get_board(false));

  for _ in 0..episodes {
    while game.judge().is_none() {
      let player = game.get_current_player();

      if player {
        let action = red.choose_action(&game);
        game.drop_disc(action, player);
        let next_state = (game.get_board(true), game.get_board(false));
        let reward = game.get_reward(player);
        red.update_q_values(&state, action, reward, &next_state);
        state = next_state;
      } else {
        let action = yellow.choose_action(&game);
        game.drop_disc(action, player);
        let next_state = (game.get_board(true), game.get_board(false));
        let reward = game.get_reward(player);
        red.update_q_values(&state, action, reward, &next_state);
        state = next_state;
      }
    }
    game.reset();
  }
  red.save_to_file(r_file_path);
  // yellow.save_to_file(y_file_path);

  println!(
    "Trained Q-learning agents and saved to {} and {}",
    r_file_path, y_file_path
  );
}
