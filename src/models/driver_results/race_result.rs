use super::lap::Lap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Race {
    pub track_name: String,
    pub date: String,
    pub session_id: u32,
    pub laptimes: Vec<Lap>,
}

impl Race {
    pub fn get_laptimes(&self) -> String {
        let mut laptimes = "".to_string();

        for lap in &self.laptimes {
            let lap = format!("Lap: {} | Time: {}", lap.number, lap.time).to_string();
            laptimes += &lap;
        }

        laptimes
    }
}
