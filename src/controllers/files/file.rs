use crate::models::application::karting_time::KartingTime;

use super::file_io::{
    read_application_state_json, read_application_state_toml, upsert_application_state_json,
    upsert_application_state_toml,
};

const APPLICATION_STATE_FILE_NAME_JSON: &str = "karting_time_state.json";
const APPLICATION_STATE_FILE_NAME_TOML: &str = "karting_time_state.toml";

impl KartingTime {
    pub fn file_new(&mut self) {
        *self = KartingTime::default();
    }

    pub fn file_save_json(&self) {
        upsert_application_state_json(APPLICATION_STATE_FILE_NAME_JSON, self);
    }

    pub fn file_load_json(&mut self) {
        *self = read_application_state_json(APPLICATION_STATE_FILE_NAME_JSON)
    }

    pub fn file_save_toml(&self) {
        upsert_application_state_toml(APPLICATION_STATE_FILE_NAME_TOML, self);
    }

    pub fn file_load_toml(&mut self) {
        *self = read_application_state_toml(APPLICATION_STATE_FILE_NAME_TOML)
    }
}

#[cfg(test)]
mod file_should {
    use crate::{
        controllers::files::file::{
            APPLICATION_STATE_FILE_NAME_JSON, APPLICATION_STATE_FILE_NAME_TOML,
        },
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
        };

        // When
        karting_time.file_new();

        // Then
        assert_eq!(expected, karting_time);
    }

    #[test]
    fn file_saves_and_loads_acceptance_test_json() {
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
        };

        // When
        karting_time.file_save_json();
        karting_time.file_load_json();

        // Then
        assert!(fs::metadata(APPLICATION_STATE_FILE_NAME_JSON).is_ok());
        assert!(
            fs::metadata(APPLICATION_STATE_FILE_NAME_JSON)
                .unwrap()
                .len()
                != 0
        );
        assert_eq!(expected, karting_time);
    }

    #[test]
    fn file_saves_and_loads_acceptance_test_toml() {
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
        };

        // When
        karting_time.file_save_toml();
        karting_time.file_load_toml();

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
