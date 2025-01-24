use serde::{Deserialize, Serialize};

use crate::models::driver_results::race_result::Race;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DriverProfile {
    pub driver_id: u32,
    pub name: String,
    pub races: Vec<Race>,
}
