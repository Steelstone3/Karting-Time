use super::race_result::Race;
use crate::data_models::driver_profile_file::DriverProfileFile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriverProfile {
    pub name: String,
    pub races: Vec<Race>,
}

impl Default for DriverProfile {
    fn default() -> Self {
        Self {
            name: "Racer".to_string(),
            races: Default::default(),
        }
    }
}

impl DriverProfile {
    pub fn convert_to_driver_profile_file(&self) -> DriverProfileFile {
        let mut race_files = vec![];

        for race in &self.races {
            race_files.push(race.convert_to_race_file())
        }

        DriverProfileFile {
            name: self.name.to_string(),
            races: race_files,
        }
    }
}

#[cfg(test)]
mod driver_profile_should {
    use super::*;
    use crate::{
        data_models::race_file::RaceFile,
        models::{
            date::Date,
            driver::{lap::Lap, race_information::RaceInformation},
        },
    };

    #[test]
    fn create_driver_profile() {
        // Given
        let expected_driver_profile = DriverProfile {
            name: "Racer".to_string(),
            races: Default::default(),
        };

        // Then
        assert_eq!(expected_driver_profile, DriverProfile::default())
    }

    #[test]
    fn convert_to_driver_profile_file() {
        // Given
        let expected_driver_profile_file = DriverProfileFile {
            name: "Karl Chadwick".to_string(),
            races: vec![RaceFile {
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
                },
                laptimes: vec!["50.662".to_string(), "51.877".to_string()],
            }],
        };

        let driver_profile = DriverProfile {
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
            }],
        };

        // When
        let driver_profile_file = driver_profile.convert_to_driver_profile_file();

        // Then
        assert_eq!(expected_driver_profile_file, driver_profile_file)
    }
}
