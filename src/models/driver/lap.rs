use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Lap {
    pub lap_number: u32,
    pub time: f32,
}
