use crate::models::driver::session_information::acc_lap::AccLap;
use serde::Deserialize;
use std::collections::BTreeMap;

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
    pub fn convert_to_laptimes(&self) -> Vec<Vec<String>> {
        let mut grouped: BTreeMap<u32, Vec<String>> = BTreeMap::new();
        let converted_laptimes = self.convert_laptime_to_seconds();

        for lap in &converted_laptimes {
            grouped
                .entry(lap.driver_index)
                .or_default()
                .push(lap.laptime.to_string().clone());
        }

        grouped.into_values().collect()
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
    fn convert_to_laptimes_empty_test() {
        // Given
        let acc_laps = vec![];
        let acc_session_data = AccSessionData {
            laps: acc_laps,
            ..Default::default()
        };

        // When
        let acc_laps_by_driver = acc_session_data.convert_to_laptimes();

        // Then
        pretty_assertions::assert_eq!(0, acc_laps_by_driver.len());
    }

    #[test]
    fn convert_to_laptimes_test() {
        // Given
        let acc_laps = vec![
            AccLap::new(0, 32200.0),
            AccLap::new(1, 31200.0),
            AccLap::new(2, 33200.0),
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
        pretty_assertions::assert_eq!("33.2".to_string(), acc_laps_by_driver[2][0]);
    }
}
