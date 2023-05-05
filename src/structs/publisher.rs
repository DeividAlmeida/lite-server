use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Publisher {
  pub id: Option<u8>,
  pub name: String,
  pub r#type: u8,
  pub gender: String,
  pub amount: Option<u32>,
  pub active: Option<bool>,
  pub updated_at: Option<String>,
  pub created_at: Option<String>,
}