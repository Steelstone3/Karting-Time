use crate::models::driver::session_information::acc_lap::AccLap;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

#[derive(Default, Debug, Deserialize)]
pub struct AccSessionData {
    #[serde(rename = "trackName")]
    pub track_name: String,
    #[serde(rename = "sessionType")]
    pub session_type: String,
    #[serde(rename = "laps")]
    pub laps: Vec<AccLap>,
}

impl AccSessionData {
    pub fn group_laps_by_driver(&self) -> BTreeMap<u32, Vec<AccLap>> {
        let mut map: BTreeMap<u32, Vec<AccLap>> = BTreeMap::new();

        for lap in &self.laps {
            map.entry(lap.driver_index).or_default().push(lap.clone());
        }

        map
    }

    pub fn calculate_race_position(&self, driver_index: u32) -> u32 {
        // TODO identify qualifying and race session from real file
        if self.session_type == "FP" || self.session_type == "Q" {
            let fastest_drivers = self.get_acc_fastest_laps();

            if let Some(position) =
                AccSessionData::find_driver_position(&fastest_drivers, driver_index)
            {
                return position;
            }

            999
        } else if self.session_type == "R" {
            let fastest_drivers = self.get_acc_total_laps();

            if let Some(position) =
                AccSessionData::find_driver_position(&fastest_drivers, driver_index)
            {
                return position;
            }

            999
        } else {
            999
        }
    }

    fn find_driver_position(laps: &[(u32, f32)], driver_index: u32) -> Option<u32> {
        laps.iter()
            .position(|(index, _)| *index == driver_index)
            .map(|position| position as u32 + 1)
    }

    fn get_acc_fastest_laps(&self) -> Vec<(u32, f32)> {
        let mut fastest_laps: HashMap<u32, f32> = HashMap::new();

        for lap in &self.laps {
            fastest_laps
                .entry(lap.driver_index)
                .and_modify(|t| {
                    if lap.laptime < *t {
                        *t = lap.laptime;
                    }
                })
                .or_insert(lap.laptime);
        }

        let mut fastest_drivers: Vec<(u32, f32)> = fastest_laps.into_iter().collect();

        fastest_drivers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        fastest_drivers
    }

    fn get_acc_total_laps(&self) -> Vec<(u32, f32)> {
        let mut total_times: HashMap<u32, f32> = HashMap::new();

        for lap in &self.laps {
            total_times
                .entry(lap.driver_index)
                .and_modify(|total_time| *total_time += lap.laptime)
                .or_insert(lap.laptime);
        }

        let mut fastest_drivers: Vec<(u32, f32)> = total_times.into_iter().collect();

        fastest_drivers.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        fastest_drivers
    }
}

#[cfg(test)]
mod acc_session_data_should {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("FP".to_string(), 999)]
    #[case("Q".to_string(), 999)]
    #[case("R".to_string(), 999)]
    #[case("".to_string(), 999)]
    fn test_calculate_race_position_with_no_laps(
        #[case] session_type: String,
        #[case] expected_driver_position: u32,
    ) {
        // Given
        let acc_laps = vec![];
        let acc_session_data = AccSessionData {
            laps: acc_laps,
            session_type,
            ..Default::default()
        };

        // When
        let driver_position = acc_session_data.calculate_race_position(0);

        // Then
        pretty_assertions::assert_eq!(expected_driver_position, driver_position);
    }

    #[rstest]
    #[case("FP".to_string(), 0, 1)]
    #[case("FP".to_string(), 1, 3)]
    #[case("FP".to_string(), 2, 2)]
    #[case("Q".to_string(), 0, 1)]
    #[case("Q".to_string(), 1, 3)]
    #[case("Q".to_string(), 2, 2)]
    #[case("R".to_string(), 0, 3)]
    #[case("R".to_string(), 1, 2)]
    #[case("R".to_string(), 2, 1)]
    #[case("".to_string(), 0, 999)]
    #[case("".to_string(), 1, 999)]
    #[case("".to_string(), 2, 999)]
    fn test_calculate_race_position(
        #[case] session_type: String,
        #[case] driver_index: u32,
        #[case] expected_driver_position: u32,
    ) {
        // Given
        let acc_laps = vec![
            AccLap::new(0, 55.0),
            AccLap::new(0, 9.0),
            AccLap::new(1, 16.0),
            AccLap::new(1, 18.0),
            AccLap::new(2, 11.0),
            AccLap::new(2, 12.0),
        ];
        let acc_session_data = AccSessionData {
            laps: acc_laps,
            session_type,
            ..Default::default()
        };

        // When
        let driver_position = acc_session_data.calculate_race_position(driver_index);

        // Then
        pretty_assertions::assert_eq!(expected_driver_position, driver_position);
    }
}
