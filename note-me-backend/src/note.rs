
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub text: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}    

