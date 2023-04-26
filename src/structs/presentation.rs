use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Presentation {
    pub id: u8,
    pub main: String,
    pub helper: String,
    pub created_at: Option<i64>,
}
