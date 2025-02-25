use crate::models::application::karting_time::KartingTime;
use crate::models::driver_results::race_file::RaceFile;
use crate::models::driver_results::race_information::RaceInformation;
use crate::models::driver_results::race_result::Race;
use std::fs::File;
use std::io::{Read, Write};

const FILE_ERROR: &str = "failed to create file";
const CONVERT_ERROR: &str = "failed to convert to toml";
const WRITE_ERROR: &str = "failed to write to file";

// TODO Test
pub fn upsert_races(file_location: &str, races: &Vec<Race>) {
    for race in races {
        let race_file = race.convert_to_race_file();

        let file_name = format!(
            "{}/{}.toml",
            file_location,
            RaceInformation::get_unique_race_identifier(&race_file.race_information)
        );

        let mut file = match File::create(file_name) {
            Ok(file) => file,
            Err(_) => {
                println!("{}", FILE_ERROR);
                return;
            }
        };

        let toml = match toml::to_string_pretty(&race_file) {
            Ok(toml) => toml,
            Err(_) => {
                println!("{}", CONVERT_ERROR);
                return;
            }
        };

        match write!(file, "{}", toml) {
            Ok(_) => (),
            Err(_) => {
                println!("{}", WRITE_ERROR);
                return;
            }
        }
    }
}

// TODO Test
pub fn read_race_file(file_name: &str) -> RaceFile {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return Default::default();
    }

    toml::from_str(&contents).unwrap_or_default()
}

pub fn upsert_application_state(file_name: &str, karting_time: &KartingTime) {
    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("{}", FILE_ERROR);
            return;
        }
    };

    let toml = match toml::to_string_pretty(&karting_time) {
        Ok(toml) => toml,
        Err(_) => {
            println!("{}", CONVERT_ERROR);
            "".to_string()
        }
    };

    match write!(file, "{}", toml) {
        Ok(file) => file,
        Err(_) => {
            println!("{}", WRITE_ERROR);
        }
    }
}

pub fn read_application_state(file_name: &str) -> KartingTime {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return KartingTime::default();
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
    use std::fs;

    #[test]
    fn read_application_state_empty() {
        // Given
        let file_name = "non_existant_file.toml";
        let expected_application = KartingTime::default();

        // When
        let karting_time = read_application_state(file_name);

        // Then
        assert_eq!(expected_application, karting_time);
    }

    #[test]
    fn upsert_application_state_to_file() {
        // Given
        let dive_planner_state_file_name = "test_file_1.toml";
        let dive_planner = KartingTime::default();

        // When
        upsert_application_state(dive_planner_state_file_name, &dive_planner);

        // Then
        assert!(fs::metadata(dive_planner_state_file_name).is_ok());
        assert!(fs::metadata(dive_planner_state_file_name).unwrap().len() != 0);
    }

    #[test]
    fn acceptance_test_read_application_state_from_file() {
        // Given
        let file_name = "test_file_2.toml";
        let expected_dive_planner = KartingTime::default();

        // When
        upsert_application_state(file_name, &expected_dive_planner);
        let dive_planner = read_application_state(file_name);

        // Then
        assert_eq!(expected_dive_planner, dive_planner);
    }
}
