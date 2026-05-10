use std::collections::{HashMap, HashSet};

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
        // self.laps
        //     .clone()
        //     .into_iter()
        //     // Convert from milliseconds
        //     .map(|lap| (lap.laptime / 1000.0).to_string())
        //     .collect()

        // TODO REMOVE AI SLOP SUGGESTION
        // let mut result = Vec::new();
        // let mut laps_for_driver = Vec::new();

        // for lap in &self.laps {
        //     if lap.driver_index == driver_index {
        //         let secs = lap.laptime / 1000.0;
        //         laps_for_driver.push(format!("{:.3}", secs));
        //     }
        // }

        // result.push(laps_for_driver);
        // result

        let unique_driver_indexs: HashSet<u32> = self
            .laps
            .clone()
            .into_iter()
            .map(|lap| lap.driver_index)
            .collect();
        let grouped_laps: Vec<Vec<String>> = vec![];

        for unique_driver_index in unique_driver_indexs {
            
        }

        grouped_laps
    }

    fn convert_laptime_to_seconds(&self) -> Vec<AccLap> {
        self.laps
            .iter()
            .map(|lap| AccLap {
                driver_index: lap.driver_index,
                laptime: lap.laptime / 1000.0,
            })
            .collect()
    }
}

#[cfg(test)]
mod acc_session_data_should {
    use super::*;

    #[test]
    fn convert_to_laptimes_test() {
        // Given
        let acc_laps = vec![
            AccLap::new(0, 3220.0),
            AccLap::new(1, 3102.0),
            AccLap::new(2, 330002.0),
        ];
        let acc_session_data = AccSessionData {
            laps: acc_laps,
            ..Default::default()
        };

        // When
        let acc_laps_by_driver = acc_session_data.convert_to_laptimes();

        // Then
        pretty_assertions::assert_eq!(3, acc_laps_by_driver.len());
        pretty_assertions::assert_eq!("32.2".to_string(), acc_laps_by_driver[0][0]);
        pretty_assertions::assert_eq!("31.2".to_string(), acc_laps_by_driver[1][0]);
        // pretty_assertions::assert_eq!("33.2".to_string(), acc_laps_by_driver[2][0]);
    }
}
