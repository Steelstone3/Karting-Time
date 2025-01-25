use crate::models::application::karting_time::KartingTime;

use super::file_io::{
    read_application_state, read_driver_profile, upsert_application_state, upsert_driver_profile,
};

const APPLICATION_STATE_FILE_NAME_TOML: &str = "karting_time_state.toml";

impl KartingTime {
    pub fn file_new(&mut self) {
        *self = KartingTime::default();
    }

    // TODO Test
    pub fn save_driver_profile(&self) {
        upsert_driver_profile(&self.driver_profile);
    }

    // TODO Test
    pub fn load_driver_profile(&mut self) {
        let mut driver_profile = read_driver_profile(&self.driver_profile);

        self.driver_profile.races.append(&mut driver_profile.races);
    }

    pub fn save_application(&self) {
        upsert_application_state(APPLICATION_STATE_FILE_NAME_TOML, self);
    }

    pub fn load_application(&mut self) {
        *self = read_application_state(APPLICATION_STATE_FILE_NAME_TOML)
    }
}

#[cfg(test)]
mod file_should {
    use crate::{
        controllers::files::file::APPLICATION_STATE_FILE_NAME_TOML,
        models::{
            application::{application_state::ApplicationState, karting_time::KartingTime},
            date::Date,
            driver_profile::profile::DriverProfile,
            driver_results::{lap::Lap, race_result::Race},
        },
    };
    use std::fs;

    #[test]
    fn new_karting_time_default_state() {
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
                    date: Date {
                        day: 12,
                        month: 12,
                        year: 2025,
                    },
                    session_id: 1,
                    race_position: 1,
                    laptimes: vec![
                        Lap {
                            lap_number: 1,
                            time: 50.4,
                        },
                        Lap {
                            lap_number: 2,
                            time: 55.5,
                        },
                    ],
                }],
            },
            ..Default::default()
        };

        // When
        karting_time.file_new();

        // Then
        assert_eq!(expected, karting_time);
    }

    #[test]
    fn acceptance_test_application_saves_then_loads() {
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
                    date: Date {
                        day: 12,
                        month: 12,
                        year: 2025,
                    },
                    session_id: 1,
                    race_position: 1,
                    laptimes: vec![
                        Lap {
                            lap_number: 1,
                            time: 50.4,
                        },
                        Lap {
                            lap_number: 2,
                            time: 55.5,
                        },
                    ],
                }],
            },
            ..Default::default()
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
                    date: Date {
                        day: 12,
                        month: 12,
                        year: 2025,
                    },
                    session_id: 1,
                    race_position: 1,
                    laptimes: vec![
                        Lap {
                            lap_number: 1,
                            time: 50.4,
                        },
                        Lap {
                            lap_number: 2,
                            time: 55.5,
                        },
                    ],
                }],
            },
            ..Default::default()
        };

        // When
        karting_time.save_application();
        karting_time.load_application();

        // Then
        assert!(fs::metadata(APPLICATION_STATE_FILE_NAME_TOML).is_ok());
        assert!(
            fs::metadata(APPLICATION_STATE_FILE_NAME_TOML)
                .unwrap()
                .len()
                != 0
        );
        assert_eq!(expected, karting_time);
    }
}
