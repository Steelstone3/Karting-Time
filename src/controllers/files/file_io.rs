use std::fs::File;
use std::io::{Read, Write};

use crate::models::application::karting_time::KartingTime;
use crate::models::driver_profile::profile::DriverProfile;

// const PATH: &str = "/files/drivers";

// pub fn create_file_repository() {
//     fs::create_dir_all(PATH).expect("Can't create repository");
// }

// TODO Test
pub fn upsert_driver_profile(driver_profile: &DriverProfile) {
    let mut file = File::create(driver_profile.create_file_path()).expect("Can't create file.");
    let toml =
        toml::to_string_pretty(&driver_profile).expect("Can't parse application data to string");
    write!(file, "{}", toml).expect("Can't update file with application data");
}

// TODO Test
pub fn read_driver_profile(driver_profile: &DriverProfile) -> DriverProfile {
    let contents = get_file_contents(&driver_profile.create_file_path());

    if contents.is_empty() {
        return Default::default();
    }

    toml::from_str(&contents).expect("Can't parse file contents to application data")
}

pub fn upsert_application_state(file_name: &str, karting_time: &KartingTime) {
    let mut file = File::create(file_name).expect("Can't create file.");
    let toml =
        toml::to_string_pretty(&karting_time).expect("Can't parse application data to string");
    write!(file, "{}", toml).expect("Can't update file with application data");
}

pub fn read_application_state(file_name: &str) -> KartingTime {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return KartingTime::default();
    }

    toml::from_str(&contents).expect("Can't parse file contents to application data")
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
    fn read_application_state_empty() {
        // Given
        let file_name = "non_existant_file.toml";
        let expected_application = KartingTime::default();

        // When
        let dive_planner = read_application_state(file_name);

        // Then
        assert_eq!(expected_application, dive_planner);
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
