use crate::models::application::karting_time::KartingTime;

use super::file_io::{read_application_state, read_race, upsert_application_state, upsert_races};

impl KartingTime {
    pub fn file_new(&mut self) {
        *self = KartingTime::default();
    }

    // TODO Test
    pub fn save_races(&self, file_location: &str) {
        upsert_races(file_location, &self.driver_profile.races);
    }

    // TODO Test
    pub fn import_race(&mut self, file_names: Vec<String>) {
        for file_name in file_names {
            let race = read_race(&file_name);

            if race.is_unique_identifer(&self.driver_profile.races) {
                self.driver_profile.races.push(race);
            }
        }
    }

    pub fn save_application(&self, file_name: &str) {
        upsert_application_state(file_name, self);
    }

    pub fn load_application(&mut self, file_name: &str) {
        *self = read_application_state(file_name)
    }
}

#[cfg(test)]
mod file_should {
    use crate::models::{
        application::{application_state::ApplicationState, karting_time::KartingTime},
        date::Date,
        driver_profile::profile::DriverProfile,
        driver_results::{lap::Lap, race_information::RaceInformation, race_result::Race},
    };
    use std::fs;

    const APPLICATION_STATE_FILE_NAME_TOML: &str = "karting_time_state.toml";

    #[test]
    fn new_karting_time_default_state() {
        // Given
        let expected = KartingTime::default();
        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                ..Default::default()
            },
            driver_profile: DriverProfile {
                name: "Jack Jackson".to_string(),
                races: vec![Race {
                    race_information: RaceInformation {
                        track_name: "Three Sisters".to_string(),
                        date: Date {
                            day: 12,
                            month: 12,
                            year: 2025,
                        },
                        session_id: 1,
                        race_position: 1,
                    },
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
                name: "Jack Jackson".to_string(),
                races: vec![Race {
                    race_information: RaceInformation {
                        track_name: "Three Sisters".to_string(),
                        date: Date {
                            day: 12,
                            month: 12,
                            year: 2025,
                        },
                        session_id: 1,
                        race_position: 1,
                    },
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
                name: "Jack Jackson".to_string(),
                races: vec![Race {
                    race_information: RaceInformation {
                        track_name: "Three Sisters".to_string(),
                        date: Date {
                            day: 12,
                            month: 12,
                            year: 2025,
                        },
                        session_id: 1,
                        race_position: 1,
                    },
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
        karting_time.save_application(APPLICATION_STATE_FILE_NAME_TOML);
        karting_time.load_application(APPLICATION_STATE_FILE_NAME_TOML);

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
