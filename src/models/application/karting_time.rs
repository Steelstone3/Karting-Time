use crate::{
    data_models::karting_time_file::KartingTimeFile, models::driver::driver_profile::DriverProfile,
};

use super::application_state::ApplicationState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct KartingTime {
    #[serde(skip)]
    pub application_state: ApplicationState,
    pub driver_profile: DriverProfile,
}

impl KartingTime {
    pub fn convert_to_karting_time_file(&self) -> KartingTimeFile {
        KartingTimeFile {
            driver_profile: self.driver_profile.convert_to_driver_profile_file(),
        }
    }
}

#[cfg(test)]
mod karting_time_should {
    use crate::{
        data_models::{driver_profile_file::DriverProfileFile, race_file::RaceFile, race_information_file::RaceInformationFile},
        models::{
            date::Date,
            driver::{lap::Lap, race_information::RaceInformation, race_result::Race},
        },
    };

    use super::*;

    #[test]
    fn convert_to_karting_time_file() {
        // Given
        let expected_karting_time_file = KartingTimeFile {
            driver_profile: DriverProfileFile {
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
                        notes: Some("Notes".to_string())
                    },
                    laptimes: vec!["50.662".to_string(), "51.877".to_string()],
                }],
            },
        };

        let karting_time = KartingTime {
            driver_profile: DriverProfile {
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
                        notes: "Notes".to_string()
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
            },
            application_state: Default::default(),
        };

        // When
        let karting_time_file = karting_time.convert_to_karting_time_file();

        // Then
        assert_eq!(expected_karting_time_file, karting_time_file)
    }
}
