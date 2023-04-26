use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Publisher {
  pub id: Option<u8>,
  pub name: String,
  pub r#type: u8,
  pub gender: String,
  pub amount: Option<u32>,
  pub active: Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>,
}