use std::fs::File;
use std::io::{Read, Write};

use crate::models::application::karting_time::KartingTime;

pub fn upsert_application_state(file_name: &str, karting_time: &KartingTime) {
    let mut file = File::create(file_name).expect("Can't create file.");
    let json = serde_json::ser::to_string_pretty(&karting_time)
        .expect("Can't parse application data to string");
    write!(file, "{}", json).expect("Can't update file with application data");
}

pub fn read_application_state(file_name: &str) -> KartingTime {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return KartingTime::default();
    }

    serde_json::from_str(&contents).expect("Can't parse file contents to application data")
}

fn get_file_contents(file_name: &str) -> String {
    let mut contents = String::new();

    if let Ok(mut file) = File::open(file_name) {
        file.read_to_string(&mut contents).expect("Can't read file");
    }

    contents
}

#[cfg(test)]
mod file_integration_should {
    use super::*;
    use std::fs;

    #[test]
    fn save_dive_planner_state_file() {
        // Given
        let dive_planner_state_file_name = "test_file_1.json";
        let dive_planner = KartingTime::default();

        // When
        upsert_application_state(dive_planner_state_file_name, &dive_planner);

        // Then
        assert!(fs::metadata(dive_planner_state_file_name).is_ok());
        assert!(fs::metadata(dive_planner_state_file_name).unwrap().len() != 0);
    }

    #[test]
    fn handle_loading_an_empty_dive_planner_state() {
        // Given
        let file_name = "non_existant_file.json";
        let expected_dive_planner = KartingTime::default();

        // When
        let dive_planner = read_application_state(file_name);

        // Then
        assert_eq!(expected_dive_planner, dive_planner);
    }

    #[test]
    fn create_a_file_saving_and_loading_dive_planner_state() {
        // Given
        let file_name = "test_file_2.json";
        let expected_dive_planner = KartingTime::default();

        // When
        upsert_application_state(file_name, &expected_dive_planner);
        let dive_planner = read_application_state(file_name);

        // Then
        assert_eq!(expected_dive_planner, dive_planner);
    }
}
