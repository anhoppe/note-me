
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
    pub title: String,
    pub text: String,
    pub date: String,
}    

