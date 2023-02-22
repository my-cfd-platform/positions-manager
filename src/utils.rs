use chrono::{DateTime, Utc};
use uuid::Uuid;

pub fn generate_position_id() -> String{
    Uuid::new_v4().to_string()
}

pub fn get_current_date() -> DateTime<Utc>{
    Utc::now()
}