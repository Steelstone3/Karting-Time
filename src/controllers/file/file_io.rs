use crate::data_models::karting_time_file::KartingTimeFile;
use crate::data_models::race_file::RaceFile;
use crate::data_models::race_information_file::RaceInformationFile;
use crate::models::driver::race_result::Race;
use std::fs::File;
use std::io::{Read, Write};

const FILE_ERROR: &str = "failed to create file";
const CONVERT_ERROR: &str = "failed to convert to toml";
const WRITE_ERROR: &str = "failed to write to file";

pub fn upsert_races(file_location: &str, races: &Vec<Race>) {
    for race in races {
        let race_file = race.convert_to_race_file();

        let file_name = format!(
            "{}/{}.toml",
            file_location,
            RaceInformationFile::get_unique_race_information_file_identifier(
                &race_file.race_information
            )
        );

        let mut file = match File::create(file_name) {
            Ok(file) => file,
            Err(_) => {
                println!("{FILE_ERROR}");
                return;
            }
        };

        let toml = match toml::to_string_pretty(&race_file) {
            Ok(toml) => toml,
            Err(_) => {
                println!("{CONVERT_ERROR}");
                return;
            }
        };

        match write!(file, "{toml}") {
            Ok(_) => (),
            Err(_) => {
                println!("{WRITE_ERROR}");
                return;
            }
        }
    }
}

pub fn read_race_file(file_name: &str) -> RaceFile {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return Default::default();
    }

    toml::from_str(&contents).unwrap_or_default()
}

pub fn upsert_application_state(file_name: &str, karting_time: &KartingTimeFile) {
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("{FILE_ERROR}");
            return;
        }
    };

    let toml = match toml::to_string_pretty(&karting_time) {
        Ok(toml) => toml,
        Err(_) => {
            println!("{CONVERT_ERROR}");
            "".to_string()
        }
    };

    match write!(file, "{toml}") {
        Ok(file) => file,
        Err(_) => {
            println!("{WRITE_ERROR}");
        }
    }
}

pub fn read_application_state(file_name: &str) -> KartingTimeFile {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return KartingTimeFile::default();
    }

    toml::from_str(&contents).unwrap_or_default()
}

fn get_file_contents(file_name: &str) -> String {
    let mut contents = String::new();

    if let Ok(mut file) = File::open(file_name) {
        file.read_to_string(&mut contents).unwrap_or_default();
    }

    contents
}

#[cfg(test)]
mod file_integration_should {
    use super::*;
    use crate::{
        data_models::race_information_file::RaceInformationFile,
        models::{date::Date, driver::race_information::RaceInformation},
    };
    use std::fs;

    #[test]
    fn upsert_races_test_failed_to_create_file() {
        // Given
        let file_location = "/";
        let races = vec![Race {
            race_information: RaceInformation {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 1,
                    month: 1,
                    year: 2025,
                },
                session_id: 1,
                ..Default::default()
            },
            ..Default::default()
        }];

        // When
        upsert_races(file_location, &races);

        // Then
        let file_name = "/".to_string()
            + &RaceInformation::get_unique_race_information_identifier(&races[0].race_information)
            + ".toml";
        assert!(fs::metadata(&file_name).is_err());
    }

    #[test]
    fn upsert_races_test() {
        // Given
        let file_location = ".";
        let races = vec![Race {
            race_information: RaceInformation {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 1,
                    month: 1,
                    year: 2025,
                },
                session_id: 1,
                ..Default::default()
            },
            ..Default::default()
        }];

        // When
        upsert_races(file_location, &races);

        // Then
        let file_name = "./".to_string()
            + &RaceInformation::get_unique_race_information_identifier(&races[0].race_information)
            + ".toml";
        assert!(fs::metadata(&file_name).is_ok());
        assert!(fs::metadata(&file_name).unwrap().len() != 0);
    }

    #[test]
    fn read_non_existant_race_file_test() {
        // Given
        let expected_race_file = RaceFile::default();

        // When
        let race_file = read_race_file("");

        // Then
        assert_eq!(expected_race_file, race_file);
    }

    #[test]
    fn read_race_file_test() {
        // Given
        let file_location = "./";
        let races = vec![Race {
            race_information: RaceInformation {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 17,
                    month: 5,
                    year: 2026,
                },
                session_id: 1,
                race_position: 1,
                car_used: "Kart".to_string(),
                notes: "Notes".to_string(),
            },
            ..Default::default()
        }];
        let expected_race_file = RaceFile {
            race_information: RaceInformationFile {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 17,
                    month: 5,
                    year: 2026,
                },
                session_id: 1,
                race_position: 1,
                car_used: Some("Kart".to_string()),
                notes: Some("Notes".to_string()),
            },
            ..Default::default()
        };

        // When
        upsert_races(file_location, &races);
        let file_name = "./".to_string()
            + &RaceInformation::get_unique_race_information_identifier(&races[0].race_information)
            + ".toml";
        let race_file = read_race_file(&file_name);

        // Then
        assert_eq!(expected_race_file, race_file);
    }

    #[test]
    fn read_race_file_test_no_car_used_or_notes() {
        // Given
        let file_location = "./";
        let races = vec![Race {
            race_information: RaceInformation {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 17,
                    month: 5,
                    year: 2026,
                },
                session_id: 1,
                race_position: 1,
                car_used: "".to_string(),
                notes: "".to_string(),
            },
            ..Default::default()
        }];
        let expected_race_file = RaceFile {
            race_information: RaceInformationFile {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 17,
                    month: 5,
                    year: 2026,
                },
                session_id: 1,
                race_position: 1,
                car_used: None,
                notes: None,
            },
            ..Default::default()
        };

        // When
        upsert_races(file_location, &races);
        let file_name = "./".to_string()
            + &RaceInformation::get_unique_race_information_identifier(&races[0].race_information)
            + ".toml";
        let race_file = read_race_file(&file_name);

        // Then
        assert_eq!(expected_race_file, race_file);
    }

    #[test]
    fn read_application_state_empty() {
        // Given
        let file_name = "non_existant_file.toml";
        let expected_application = KartingTimeFile::default();

        // When
        let karting_time = read_application_state(file_name);

        // Then
        assert_eq!(expected_application, karting_time);
    }

    #[test]
    fn upsert_application_state_failed_to_create_file() {
        // Given
        let karting_time_state_file_name = "/karting_time_test_file_1.toml";
        let karting_time_file = KartingTimeFile::default();

        // When
        upsert_application_state(karting_time_state_file_name, &karting_time_file);

        // Then
        assert!(fs::metadata(karting_time_state_file_name).is_err());
    }

    #[test]
    fn upsert_application_state_to_file() {
        // Given
        let karting_time_state_file_name = "karting_time_test_file_1.toml";
        let karting_time_file = KartingTimeFile::default();

        // When
        upsert_application_state(karting_time_state_file_name, &karting_time_file);

        // Then
        assert!(fs::metadata(karting_time_state_file_name).is_ok());
        assert!(fs::metadata(karting_time_state_file_name).unwrap().len() != 0);
    }

    #[test]
    fn acceptance_test_read_application_state_from_file() {
        // Given
        let file_name = "karting_time_test_file_2.toml";
        let expected_karting_time = KartingTimeFile::default();

        // When
        upsert_application_state(file_name, &expected_karting_time);
        let karting_time = read_application_state(file_name);

        // Then
        assert_eq!(expected_karting_time, karting_time);
    }
}
