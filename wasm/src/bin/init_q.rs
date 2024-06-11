use std::fs;
use wasm::q_learning::QLearningAgent; // 適切なクレート名に置き換えてください

fn main() {
  let agent = QLearningAgent::new();
  let serialized_agent = agent.serialize();
  let file_path = "red_q_table.json";

  fs::write(file_path, serialized_agent).expect("Unable to write file");
  println!("Initialized Q-learning agent and saved to {}", file_path);
}
