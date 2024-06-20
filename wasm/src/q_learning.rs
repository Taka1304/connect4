use rand::prelude::SliceRandom;
use rand::Rng;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fs;

use crate::board::BitBoard;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QLearningAgent {
  #[serde(
    serialize_with = "serialize_q_table",
    deserialize_with = "deserialize_q_table"
  )]
  pub q_table: HashMap<(u64, u64), Vec<f64>>,
  pub learning_rate: f64,
  pub discount_factor: f64,
  pub exploration_rate: f64,
}
impl Default for QLearningAgent {
  fn default() -> Self {
    QLearningAgent {
      q_table: HashMap::new(),
      learning_rate: 0.3,
      discount_factor: 0.99,
      exploration_rate: 0.2,
    }
  }
}

impl QLearningAgent {
  pub fn new(learning_rate: f64, discount_factor: f64, exploration_rate: f64) -> Self {
    QLearningAgent {
      learning_rate,
      discount_factor,
      exploration_rate,
      q_table: HashMap::new(),
    }
  }

  pub fn get_reward(self, board: &BitBoard, turn: bool) -> f64 {
    match board.judge() {
      Some(1) if turn => 1.0,
      Some(1) => -1.0,
      Some(2) if turn => -1.0,
      Some(2) => 1.0,
      _ => 0.0,
    }
  }

  pub fn get_q_values(&mut self, state: &(u64, u64)) -> &mut Vec<f64> {
    self.q_table.entry(*state).or_insert(vec![0.0; 7])
  }

  pub fn choose_action(&mut self, board: &BitBoard) -> usize {
    let state = (board.player1, board.player2);
    let mut rng = rand::thread_rng();
    let mut valid_actions: Vec<usize> = (0..7).collect();
    valid_actions.retain(|&col| !board.is_column_full(col));

    if rng.gen::<f64>() < self.exploration_rate {
      *valid_actions.choose(&mut rng).unwrap()
    } else {
      let q_values = self.get_q_values(&state);
      let max_q = valid_actions
        .iter()
        .map(|&col| &q_values[col])
        .fold(&f64::NEG_INFINITY, |a, b| if a > b { a } else { b });
      valid_actions
        .into_iter()
        .find(|&col| q_values[col] == *max_q)
        .unwrap()
    }
  }

  pub fn update_q_values(
    &mut self,
    state: &(u64, u64),
    action: usize,
    reward: f64,
    next_state: &(u64, u64),
  ) {
    let q_values = self.get_q_values(state).clone();
    let next_q_values = self.get_q_values(next_state).clone();

    let max_next_q = next_q_values
      .iter()
      .cloned()
      .fold(f64::NEG_INFINITY, f64::max);

    let q_value = q_values[action];
    let new_q_value =
      q_value + self.learning_rate * (reward + self.discount_factor * max_next_q - q_value);

    self
      .q_table
      .entry(*state)
      .and_modify(|v| v[action] = new_q_value);
  }

  pub fn serialize(&self) -> String {
    serde_json::to_string(self).unwrap()
  }

  pub fn deserialize(serialized: &str) -> Self {
    serde_json::from_str(serialized).unwrap()
  }

  pub fn save_to_file(&self, file_path: &str) {
    let serialized = self.serialize();
    fs::write(file_path, serialized).expect("Unable to write file");
  }

  pub fn load_from_file(file_path: &str) -> Self {
    match fs::read_to_string(file_path) {
      Ok(serialized) => Self::deserialize(&serialized),
      Err(_) => Self::default(),
    }
  }
}

// カスタムシリアライザ関数
fn serialize_q_table<S>(
  q_table: &HashMap<(u64, u64), Vec<f64>>,
  serializer: S,
) -> Result<S::Ok, S::Error>
where
  S: serde::Serializer,
{
  let mut map = serializer.serialize_map(Some(q_table.len()))?;
  for (key, value) in q_table {
    let key_string = format!("{},{}", key.0, key.1);
    map.serialize_entry(&key_string, value)?;
  }
  map.end()
}

// カスタムデシリアライザ関数
fn deserialize_q_table<'de, D>(deserializer: D) -> Result<HashMap<(u64, u64), Vec<f64>>, D::Error>
where
  D: Deserializer<'de>,
{
  struct QTableVisitor;

  impl<'de> Visitor<'de> for QTableVisitor {
    type Value = HashMap<(u64, u64), Vec<f64>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      formatter.write_str("a map with string keys and array values")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
      M: MapAccess<'de>,
    {
      let mut map = HashMap::new();
      while let Some((key, value)) = access.next_entry::<String, Vec<f64>>()? {
        let parts: Vec<&str> = key.split(',').collect();
        if parts.len() == 2 {
          if let (Ok(k1), Ok(k2)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
            map.insert((k1, k2), value);
          }
        }
      }
      Ok(map)
    }
  }

  deserializer.deserialize_map(QTableVisitor)
}
