use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestData {
  pub length: u8,
  pub gender: String,
}