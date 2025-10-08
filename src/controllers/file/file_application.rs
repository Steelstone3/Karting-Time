use super::file_io::{
    read_application_state, read_race_file, upsert_application_state, upsert_races,
};
use crate::{
    controllers::file::file_io::upsert_html_races, models::application::karting_time::KartingTime,
};

impl KartingTime {
    pub fn file_new(&mut self) {
        *self = KartingTime::default();
    }

    pub fn export_races(&self, folder_location: &str) {
        upsert_races(folder_location, &self.driver_profile.races);
    }

    pub fn export_html_races(&self, folder_location: &str) {
        upsert_html_races(folder_location, &self.driver_profile);
    }

    pub fn import_races(&mut self, file_names: Vec<String>) {
        for file_name in file_names {
            let race_file = read_race_file(&file_name);

            let race = race_file.convert_to_race_result();

            if race.is_unique_identifier(&self.driver_profile.races) {
                self.driver_profile.races.push(race);
            }
        }

        self.driver_profile.update_driver_profile();
    }

    pub fn save_application(&self, file_path: &str) {
        let karting_time_file = self.convert_to_karting_time_file();

        upsert_application_state(file_path, &karting_time_file);
    }

    pub fn load_application(&mut self, file_name: &str) {
        let karting_time_file = read_application_state(file_name);

        *self = karting_time_file.convert_to_karting_time();
    }
}

#[cfg(test)]
mod file_application_should {
    use super::*;
    use crate::{
        controllers::file::test_file_guard::TestFileGuard,
        models::{
            date::RaceDate,
            driver::{
                driver_profile::DriverProfile,
                session_information::{
                    lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
                    race_result::RaceResult, session::Session,
                },
            },
        },
    };
    use std::fs;

    #[test]
    fn export_races_test() {
        // Given
        let file_location = ".";
        let karting_time = KartingTime::new(driver_profile_test_fixture());

        // When
        karting_time.export_races(file_location);

        // Then
        let file_name_1 = "./".to_string()
            + &karting_time.driver_profile.races[0]
                .race_information
                .unique_race_identifier
            + ".toml";

        let file_name_2 = "./".to_string()
            + &karting_time.driver_profile.races[1]
                .race_information
                .unique_race_identifier
            + ".toml";

        let _guard = TestFileGuard::new(&file_name_1);
        let _guard = TestFileGuard::new(&file_name_2);

        assert!(fs::metadata(&file_name_1).is_ok());
        assert!(fs::metadata(&file_name_1).unwrap().len() != 0);
        assert!(fs::metadata(&file_name_2).is_ok());
        assert!(fs::metadata(&file_name_2).unwrap().len() != 0);
    }

    #[test]
    fn export_races_html_test() {
        // Given
        let file_location = ".";
        let karting_time = KartingTime::new(driver_profile_test_fixture());

        // When
        karting_time.export_html_races(file_location);

        // Then
        let file_name = format!("./{}.html", &karting_time.driver_profile.name);
        let _guard = TestFileGuard::new(&file_name);
        assert!(fs::metadata(&file_name).is_ok());
        assert_ne!(fs::metadata(&file_name).unwrap().len(), 0);
    }

    #[test]
    fn import_non_existent_race_test() {
        // Given
        let expected_race = RaceResult::default();
        let mut karting_time = KartingTime::default();

        // When
        karting_time.import_races(vec!["".to_string()]);

        // Then
        pretty_assertions::assert_eq!(expected_race, karting_time.driver_profile.races[0]);
    }

    #[test]
    fn import_races_test() {
        // Given
        let mut karting_time = KartingTime::default();
        let file_location = "./";
        let races = vec![RaceResult::new(
            RaceInformation::new(
                "Three Sisters",
                RaceDate::new(17, 10, 2027),
                Session::new(1, 1),
            ),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                "Kart",
                "Championship",
                "Notes",
            ),
            Default::default(),
        )];

        // When
        let file_name =
            "./".to_string() + &races[0].race_information.unique_race_identifier + ".toml";

        let _guard = TestFileGuard::new(&file_name);

        upsert_races(file_location, &races);

        karting_time.import_races(vec![file_name]);

        // Then
        pretty_assertions::assert_eq!(races[0], karting_time.driver_profile.races[0]);
    }

    #[test]
    fn new_karting_time_default_state() {
        // Given
        let expected = KartingTime::default();
        let mut karting_time = KartingTime::new(driver_profile_test_fixture());

        // When
        karting_time.file_new();

        // Then
        pretty_assertions::assert_eq!(expected, karting_time);
    }

    #[test]
    fn acceptance_test_application_saves_then_loads() {
        // Given
        let file_name = "./karting_time_state.toml";
        let expected = KartingTime::new(driver_profile_test_fixture());
        let mut karting_time = KartingTime::new(driver_profile_test_fixture());

        // When
        let _guard = TestFileGuard::new(&file_name);

        karting_time.save_application(file_name);
        karting_time.load_application(file_name);

        // Then
        assert!(fs::metadata(file_name).is_ok());
        assert!(fs::metadata(file_name).unwrap().len() != 0);
        pretty_assertions::assert_eq!(expected, karting_time);
    }

    fn driver_profile_test_fixture() -> DriverProfile {
        DriverProfile::new(
            "Jack Jackson",
            vec![
                RaceResult::new(
                    RaceInformation::new(
                        "Three Sisters",
                        RaceDate::new(12, 12, 2025),
                        Session::new(1, 1),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    vec![Lap::new(1, 50.4), Lap::new(2, 55.5)],
                ),
                RaceResult::new(
                    RaceInformation::new(
                        "Trafford Park",
                        RaceDate::new(15, 1, 2024),
                        Session::new(2, 3),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    vec![Lap::new(1, 56.8), Lap::new(2, 58.7)],
                ),
            ],
        )
    }
}
