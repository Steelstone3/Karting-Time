use crate::models::application::karting_time::KartingTime;

use super::file_io::{read_application_state, upsert_application_state};

const APPLICATION_STATE_FILE_NAME: &str = "karting_time_state.json";

impl KartingTime {
    pub fn file_new(&mut self) {
        *self = KartingTime::default();
    }

    pub fn file_save(&self) {
        upsert_application_state(APPLICATION_STATE_FILE_NAME, self);
    }

    pub fn file_load(&mut self) {
        *self = read_application_state(APPLICATION_STATE_FILE_NAME)
    }
}

#[cfg(test)]
mod file_should {
    use crate::{
        controllers::files::file::APPLICATION_STATE_FILE_NAME,
        models::{
            application::{application_state::ApplicationState, karting_time::KartingTime},
            driver_profile::profile::DriverProfile,
            driver_results::{lap::Lap, race_result::Race},
        },
    };
    use std::fs;

    #[test]
    fn reset_karting_time_to_default_state() {
        // Given
        let expected = KartingTime::default();
        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                ..Default::default()
            },
            driver_profile: DriverProfile {
                driver_id: 12,
                name: "Jack Jackson".to_string(),
                races: vec![Race {
                    track_name: "Three Sisters".to_string(),
                    date: "12/12/2025".to_string(),
                    session_id: 1,
                    laptimes: vec![Lap {
                        number: 1,
                        time: 50.4,
                    }],
                }],
            },
        };

        // When
        karting_time.file_new();

        // Then
        assert_eq!(expected, karting_time);
    }

    #[test]
    fn file_saves_and_loads_acceptance_test() {
        // Given
        let expected = KartingTime {
            application_state: ApplicationState {
                ..Default::default()
            },
            driver_profile: DriverProfile {
                driver_id: 12,
                name: "Jack Jackson".to_string(),
                races: vec![Race {
                    track_name: "Three Sisters".to_string(),
                    date: "12/12/2025".to_string(),
                    session_id: 1,
                    laptimes: vec![Lap {
                        number: 1,
                        time: 50.4,
                    }],
                }],
            },
        };
        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                ..Default::default()
            },
            driver_profile: DriverProfile {
                driver_id: 12,
                name: "Jack Jackson".to_string(),
                races: vec![Race {
                    track_name: "Three Sisters".to_string(),
                    date: "12/12/2025".to_string(),
                    session_id: 1,
                    laptimes: vec![Lap {
                        number: 1,
                        time: 50.4,
                    }],
                }],
            },
        };

        // When
        karting_time.file_save();
        karting_time.file_load();

        // Then
        assert!(fs::metadata(APPLICATION_STATE_FILE_NAME).is_ok());
        assert!(fs::metadata(APPLICATION_STATE_FILE_NAME).unwrap().len() != 0);
        assert_eq!(expected, karting_time);
    }
}
