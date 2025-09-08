use crate::models::application::karting_time::KartingTime;

use super::file_io::{
    read_application_state, read_race_file, upsert_application_state, upsert_races,
};

impl KartingTime {
    pub fn file_new(&mut self) {
        *self = KartingTime::default();
    }

    pub fn export_races(&self, file_location: &str) {
        upsert_races(file_location, &self.driver_profile.races);
    }

    pub fn import_race(&mut self, file_names: Vec<String>) {
        for file_name in file_names {
            let race_file = read_race_file(&file_name);

            let race = race_file.convert_to_race();

            if race.is_unique_identifer(&self.driver_profile.races) {
                self.driver_profile.races.push(race);
            }
        }
    }

    pub fn save_application(&self, file_name: &str) {
        let karting_time_file = self.convert_to_karting_time_file();

        upsert_application_state(file_name, &karting_time_file);
    }

    pub fn load_application(&mut self, file_name: &str) {
        let karting_time_file = read_application_state(file_name);

        *self = karting_time_file.convert_to_karting_time();
    }
}

#[cfg(test)]
mod file_should {
    use super::*;
    use crate::models::{
        application::application_state::ApplicationState,
        date::Date,
        driver::{
            driver_profile::DriverProfile, lap::Lap, race_information::RaceInformation,
            race_result::Race, session::Session,
        },
    };
    use std::fs;

    #[test]
    fn export_race_test() {
        // Given
        let file_location = ".";
        let karting_time = KartingTime {
            driver_profile: driver_profile_test_fixture(),
            ..Default::default()
        };

        // When
        karting_time.export_races(file_location);

        // Then
        let file_name = "./".to_string()
            + &RaceInformation::get_unique_race_information_identifier(
                &karting_time.driver_profile.races[0].race_information,
            )
            + ".toml";
        assert!(fs::metadata(&file_name).is_ok());
        assert!(fs::metadata(&file_name).unwrap().len() != 0);
    }

    #[test]
    fn import_non_existant_race_test() {
        // Given
        let expected_race = Race::default();
        let mut karting_time = KartingTime::default();

        // When
        karting_time.import_race(vec!["".to_string()]);

        // Then
        assert_eq!(expected_race, karting_time.driver_profile.races[0]);
    }

    #[test]
    fn import_races_test() {
        // Given
        let mut karting_time = KartingTime::default();
        let file_location = "./";
        let races = vec![Race {
            race_information: RaceInformation {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 17,
                    month: 10,
                    year: 2027,
                },
                session: Session {
                    race_position: 1,
                    session_id: 1,
                    session_type: "N/A".to_string(),
                    track_condition: "N/A".to_string(),
                },
                car_used: "Kart".to_string(),
                championship: "Championship".to_string(),
                notes: "Notes".to_string(),
            },
            ..Default::default()
        }];

        // When
        upsert_races(file_location, &races);
        let file_name = "./".to_string()
            + &RaceInformation::get_unique_race_information_identifier(&races[0].race_information)
            + ".toml";
        karting_time.import_race(vec![file_name]);

        // Then
        assert_eq!(races[0], karting_time.driver_profile.races[0]);
    }

    #[test]
    fn new_karting_time_default_state() {
        // Given
        let expected = KartingTime::default();
        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                ..Default::default()
            },
            driver_profile: driver_profile_test_fixture(),
        };

        // When
        karting_time.file_new();

        // Then
        assert_eq!(expected, karting_time);
    }

    #[test]
    fn acceptance_test_application_saves_then_loads() {
        // Given
        let application_state_file_name_toml = "./karting_time_state.toml";
        let expected = KartingTime {
            application_state: ApplicationState {
                ..Default::default()
            },
            driver_profile: driver_profile_test_fixture(),
        };
        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                ..Default::default()
            },
            driver_profile: driver_profile_test_fixture(),
        };

        // When
        karting_time.save_application(application_state_file_name_toml);
        karting_time.load_application(application_state_file_name_toml);

        // Then
        assert!(fs::metadata(application_state_file_name_toml).is_ok());
        assert!(
            fs::metadata(application_state_file_name_toml)
                .unwrap()
                .len()
                != 0
        );
        assert_eq!(expected, karting_time);
    }

    fn driver_profile_test_fixture() -> DriverProfile {
        DriverProfile {
            name: "Jack Jackson".to_string(),
            races: vec![
                Race {
                    race_information: RaceInformation {
                        track_name: "Three Sisters".to_string(),
                        date: Date {
                            day: 12,
                            month: 12,
                            year: 2025,
                        },
                        session: Session {
                            session_id: 1,
                            session_type: "N/A".to_string(),
                            track_condition: "N/A".to_string(),
                            race_position: 1,
                        },
                        car_used: "Kart".to_string(),
                        championship: "Championship".to_string(),
                        notes: Default::default(),
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
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        track_name: "Trafford Park".to_string(),
                        date: Date {
                            day: 15,
                            month: 1,
                            year: 2024,
                        },
                        session: Session {
                            session_id: 2,
                            session_type: "N/A".to_string(),
                            track_condition: "N/A".to_string(),
                            race_position: 3,
                        },
                        car_used: "Kart".to_string(),
                        championship: "Championship".to_string(),
                        notes: Default::default(),
                    },
                    laptimes: vec![
                        Lap {
                            lap_number: 1,
                            time: 56.8,
                        },
                        Lap {
                            lap_number: 2,
                            time: 58.7,
                        },
                    ],
                    ..Default::default()
                },
            ],
        }
    }
}
