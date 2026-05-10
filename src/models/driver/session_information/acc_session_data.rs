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
    #[serde(rename = "laps")]
    pub laps: Vec<AccLap>,
}

impl AccSessionData {
    // pub fn new(track_name: &str, session_type: &str, laps: Vec<AccLap>) -> Self {
    //     Self {
    //         track_name: track_name.to_string(),
    //         session_type: session_type.to_string(),
    //         session_index: 999,
    //         laps,
    //     }
    // }

    // // TODO test
    // pub fn extract_driver_laptimes(&self) -> Vec<AccSessionData> {
    //     vec![]
    // }

    // TODO test
    pub fn convert_to_laptimes(&self) -> Vec<Vec<String>> {
        self.laps
            .clone()
            .into_iter()
            // Convert from milliseconds
            .map(|lap| (lap.laptime / 1000.0).to_string())
            .collect()
    }
}

#[cfg(test)]
mod acc_session_data_should {
    use super::*;

    #[test]
    fn convert_to_laptimes_test() {
        // Given
        let acc_laps = vec![AccLap::new(0, 32.2)];
        let acc_session_data = AccSessionData {
            laps: acc_laps,
            ..Default::default()
        };

        // When
        let acc_laps_by_driver = acc_session_data.convert_to_laptimes();

        // Then
        pretty_assertions::assert_eq!()
    }
}
