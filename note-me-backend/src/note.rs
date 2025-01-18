
use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    #[serde(deserialize_with = "deserialize_id")]
    pub id: u64,
    
    #[serde(rename = "userId")]
    pub user_id: String,
    
    pub title: String,
    
    pub text: String,
    
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}    

fn deserialize_id<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?; // Directly deserialize as String
    s.parse::<u64>().map_err(serde::de::Error::custom) // Parse as u64
}
