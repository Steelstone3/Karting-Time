use crate::models::driver::session_information::acc_lap::AccLap;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct AccSessionData {
    #[serde(rename = "trackName")]
    pub track_name: String,
    #[serde(rename = "sessionType")]
    pub session_type: String,
    #[serde(rename = "sessionIndex")]
    pub session_index: u32,
    // TODO Then from the laptime totals each driver index the fastest to slowest can be established
    // TODO For a race total time or the fastest lap for a practise session/ qualifying
    #[serde(rename = "laps")]
    pub laps: Vec<AccLap>,
}

impl AccSessionData {
    pub fn convert_to_laptimes(&self) -> Vec<String> {
        self.laps
            .clone()
            .into_iter()
            // Convert from milliseconds
            .map(|lap| (lap.laptime / 1000.0).to_string())
            .collect()
    }
}
