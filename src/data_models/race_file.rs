use crate::{
    data_models::race_information_file::RaceInformationFile,
    models::driver::{lap::Lap, race_result::Race},
};
use serde::{Deserialize, Serialize};
use std::f32;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaceFile {
    pub race_information: RaceInformationFile,
    pub laptimes: Vec<String>,
}

impl RaceFile {
    pub fn convert_to_race(&self) -> Race {
        Race {
            race_information: self.race_information.convert_to_race_information(),
            laptimes: self.convert_laptimes_to_laps(),
            is_deleting: Default::default(),
        }
    }

    fn convert_laptimes_to_laps(&self) -> Vec<Lap> {
        let mut laps: Vec<Lap> = vec![];

        for (index, laptime) in self.laptimes.iter().enumerate() {
            let time = laptime.parse::<f32>();

            if let Ok(time) = time {
                laps.push(Lap {
                    lap_number: (index + 1) as u32,
                    time,
                })
            }
        }

        laps
    }
}

#[cfg(test)]
mod race_file_should {
    use super::*;
    use crate::{data_models::race_file::RaceFile, models::{date::Date, driver::race_information::RaceInformation}};

    #[test]
    fn convert_to_race() {
        // Given
        let expected_race = Race {
            race_information: RaceInformation {
                track_name: "Three Ponies".to_string(),
                date: Date {
                    day: 15,
                    month: 10,
                    year: 2024,
                },
                session_id: 1,
                race_position: 2,
                car_used: "Kart".to_string(),
                notes: "Notes".to_string(),
            },
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 50.662,
                },
                Lap {
                    lap_number: 2,
                    time: 51.877,
                },
            ],
            ..Default::default()
        };

        let race_file = RaceFile {
            race_information: RaceInformationFile {
                track_name: "Three Ponies".to_string(),
                date: Date {
                    day: 15,
                    month: 10,
                    year: 2024,
                },
                session_id: 1,
                race_position: 2,
                car_used: Some("Kart".to_string()),
                notes: Some("Notes".to_string()),
            },
            laptimes: vec!["50.662".to_string(), "51.877".to_string()],
        };

        // When
        let race = race_file.convert_to_race();

        // Then
        assert_eq!(expected_race, race)
    }
}
