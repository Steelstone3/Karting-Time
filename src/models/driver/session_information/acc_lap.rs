use serde::Deserialize;

#[derive(Clone, Default, Debug, Deserialize)]
pub struct AccLap {
    #[serde(rename = "driverIndex")]
    pub driver_index: u32,
    #[serde(rename = "laptime")]
    pub laptime: f32,
}
