use std::collections::HashMap;

pub struct QLearningAgent {
  pub q_table: HashMap<String, Vec<f64>>,
  pub learning_rate: f64,
  pub discount_factor: f64,
  pub exploration_rate: f64,
}

impl QLearningAgent {
  pub fn new() -> Self {
    QLearningAgent {
      q_table: HashMap::new(),
      learning_rate: 0.1,
      discount_factor: 0.99,
      exploration_rate: 0.1,
    }
  }

  pub fn get_q_values(&mut self, state: &str) -> &mut Vec<f64> {
    self
      .q_table
      .entry(state.to_string())
      .or_insert(vec![0.0; 7])
  }

  pub fn choose_action(&mut self, state: &str) -> usize {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    if rng.gen::<f64>() < self.exploration_rate {
      rng.gen_range(0..7)
    } else {
      let q_values = self.get_q_values(state);
      let max_q = q_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
      q_values.iter().position(|&q| q == max_q).unwrap()
    }
  }

  pub fn update_q_values(&mut self, state: &str, action: usize, reward: f64, next_state: &str) {
    // `q_values`を一度に借用しないように変更
    let q_values = self.get_q_values(state).clone();
    let next_q_values = self.get_q_values(next_state).clone();

    let max_next_q = next_q_values
      .iter()
      .cloned()
      .fold(f64::NEG_INFINITY, f64::max);

    let q_value = q_values[action];
    let new_q_value =
      q_value + self.learning_rate * (reward + self.discount_factor * max_next_q - q_value);

    // 結果を反映するために再度ミュータブルな借用を取得
    self
      .q_table
      .entry(state.to_string())
      .and_modify(|v| v[action] = new_q_value);
  }
}
