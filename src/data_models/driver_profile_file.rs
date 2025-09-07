use crate::models::driver::driver_profile::DriverProfile;

use super::race_file::RaceFile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DriverProfileFile {
    pub name: String,
    pub races: Vec<RaceFile>,
}

impl DriverProfileFile {
    pub fn convert_to_driver_profile(&self) -> DriverProfile {
        let mut races = vec![];

        for race_file in &self.races {
            races.push(race_file.convert_to_race());
        }

        DriverProfile {
            name: self.name.to_string(),
            races,
        }
    }
}

#[cfg(test)]
mod profile_file_should {
    use crate::{
        data_models::race_information_file::RaceInformationFile,
        models::{
            date::Date,
            driver::{lap::Lap, race_information::RaceInformation, race_result::Race},
        },
    };

    use super::*;

    #[test]
    fn convert_to_driver_profile() {
        // Given
        let expected_driver_profile = DriverProfile {
            name: "Karl Chadwick".to_string(),
            races: vec![Race {
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
            }],
        };

        let driver_profile_file = DriverProfileFile {
            name: "Karl Chadwick".to_string(),
            races: vec![RaceFile {
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
            }],
            ..Default::default()
        };

        // When
        let driver_profile = driver_profile_file.convert_to_driver_profile();

        // Then
        assert_eq!(expected_driver_profile, driver_profile)
    }
}
