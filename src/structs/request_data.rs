use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RequestData {
  length: u8,
  gender: String,
}