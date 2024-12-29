
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub text: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}    

