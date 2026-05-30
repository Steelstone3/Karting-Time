use super::file_io::{
    read_application_state, read_race_file, upsert_application_state, upsert_races,
};
use crate::{
    controllers::file::file_io::{read_acc_laptimes_file, read_laptimes_file, upsert_html_races},
    models::application::karting_time::KartingTime,
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

    pub fn import_acc_laptimes(&mut self, file_name: &str) {
        let race_files = read_acc_laptimes_file(file_name);

        for race_file in race_files {
            let Some(race_file) = race_file else { return };
            let race = race_file.convert_to_race_result();

            if race.is_unique_identifier(&self.driver_profile.races) {
                self.driver_profile.races.push(race);
            }
        }
    }

    pub fn import_laptimes(&mut self, file_name: &str) {
        let race_file = read_laptimes_file(file_name);

        let Some(race_file) = race_file else { return };
        let race = race_file.convert_to_race_result();

        if race.is_unique_identifier(&self.driver_profile.races) {
            self.driver_profile.races.push(race);
        }
    }

    pub fn import_races(&mut self, file_names: Vec<String>) {
        for file_name in file_names {
            let race_file = read_race_file(&file_name);

            let Some(race_file) = race_file else { continue };
            let race = race_file.convert_to_race_result();

            if race.is_unique_identifier(&self.driver_profile.races) {
                self.driver_profile.races.push(race);
            }

            self.driver_profile.update_driver_profile();
        }
    }

    pub fn save_application(&self, file_path: &str) {
        let karting_time_file = self.convert_to_karting_time_file();

        upsert_application_state(file_path, &karting_time_file);
    }

    pub fn load_application(&mut self, file_name: &str) {
        let karting_time_file = read_application_state(file_name);

        let Some(karting_time_file) = karting_time_file else {
            return;
        };
        *self = karting_time_file.convert_to_karting_time()
    }
}

#[cfg(test)]
mod file_application_should {
    use super::*;
    use crate::{
        controllers::file::test_file_guard::TestFileGuard,
        data_models::race_result_file::RaceResultFile,
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
    use rstest::rstest;

    #[test]
    fn test_export_races() {
        // Given
        let driver_profile = DriverProfile::new(
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
        );
        let karting_time = KartingTime::new(driver_profile);
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let file_name_1 = file_location.join(
            karting_time.driver_profile.races[0]
                .race_information
                .unique_race_identifier
                .clone()
                + ".toml",
        );
        let file_name_2 = file_location.join(
            karting_time.driver_profile.races[1]
                .race_information
                .unique_race_identifier
                .clone()
                + ".toml",
        );

        // When
        karting_time.export_races(&file_location.to_string_lossy());

        // Then
        assert!(std::path::Path::new(&file_name_1).is_file());
        assert!(std::path::Path::new(&file_name_2).is_file());

        // Cleanup
        let _guard = TestFileGuard::new(&file_name_1.to_string_lossy());
        let _guard = TestFileGuard::new(&file_name_2.to_string_lossy());
    }

    #[test]
    fn test_export_races_html() {
        // Given
        let driver_profile = DriverProfile::new(
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
        );
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let karting_time = KartingTime::new(driver_profile);
        let file_name = file_location.join(karting_time.driver_profile.name.to_string() + ".html");

        // When
        karting_time.export_html_races(&file_location.to_string_lossy());

        // Then
        assert!(std::path::Path::new(&file_name).is_file());

        // Cleanup
        let _guard = TestFileGuard::new(&file_name.to_string_lossy());
    }

    #[test]
    fn test_import_non_existent_acc_laptimes() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        karting_time.import_acc_laptimes("");

        // Then
        assert!(karting_time.driver_profile.races.is_empty());
    }

    #[test]
    fn test_import_acc_laptimes() {
        // Given
        let mut karting_time = KartingTime::default();
        let track_name = "silverstone";
        let session_1 = Session::new(1001, 3);
        let session_2 = Session::new(1002, 2);
        let session_3 = Session::new(1003, 1);
        let race_date = RaceDate::today();
        let race_information_1 = RaceInformation::new(track_name, race_date.clone(), session_1);
        let race_information_2 = RaceInformation::new(track_name, race_date.clone(), session_2);
        let race_information_3 = RaceInformation::new(track_name, race_date.clone(), session_3);
        let laptimes_1: Vec<Lap> = vec![
            Lap::new(1, 122.147),
            Lap::new(2, 121.1),
            Lap::new(3, 122.215),
            Lap::new(4, 122.18),
            Lap::new(5, 121.297),
        ];
        let laptimes_2: Vec<Lap> = vec![
            Lap::new(1, 122.505),
            Lap::new(2, 121.615),
            Lap::new(3, 121.702),
            Lap::new(4, 120.785),
        ];

        let laptimes_3: Vec<Lap> = vec![
            Lap::new(1, 121.935),
            Lap::new(2, 123.527),
            Lap::new(3, 120.522),
        ];
        let race_meta_data = RaceMetadata::new("FP", "N/A", "N/A", "", "Imported from ACC");

        let expected_race_1 =
            RaceResult::new(race_information_1, race_meta_data.clone(), laptimes_1);
        let expected_race_2 =
            RaceResult::new(race_information_2, race_meta_data.clone(), laptimes_2);
        let expected_race_3 =
            RaceResult::new(race_information_3, race_meta_data.clone(), laptimes_3);

        let file_name = "./file_io_test_files/acc_file_2.json";

        // When
        karting_time.import_acc_laptimes(file_name);

        // Then
        assert!(
            std::path::Path::new(&file_name).is_file(),
            "Expected test file to exist at path: {}",
            file_name
        );
        pretty_assertions::assert_eq!(3, karting_time.driver_profile.races.len());
        pretty_assertions::assert_eq!(5, karting_time.driver_profile.races[0].laptimes.len());
        pretty_assertions::assert_eq!(4, karting_time.driver_profile.races[1].laptimes.len());
        pretty_assertions::assert_eq!(3, karting_time.driver_profile.races[2].laptimes.len());
        pretty_assertions::assert_eq!(expected_race_1, karting_time.driver_profile.races[0]);
        pretty_assertions::assert_eq!(expected_race_2, karting_time.driver_profile.races[1]);
        pretty_assertions::assert_eq!(expected_race_3, karting_time.driver_profile.races[2]);
    }

    #[test]
    fn test_read_non_existent_laptime_file() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        karting_time.import_laptimes("");

        // Then
        assert!(karting_time.driver_profile.races.is_empty());
    }

    #[rstest]
    #[case(
        "./file_io_test_files/laptime_file_test.txt".to_string(), 
        vec!["2:00.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string()]
    )]
    #[case(
        "./file_io_test_files/laptime_file_test.csv".to_string(), 
        vec!["2:00.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string()]
    )]
    #[case(
        "./file_io_test_files/laptime_file_test.md".to_string(), 
        vec!["2:00.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string()]
    )]
    #[case(
        "./file_io_test_files/laptime_file_test_collection_1.json".to_string(), 
        vec!["120.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string()]
    )]
    #[case(
        "./file_io_test_files/laptime_file_test_collection_2.json".to_string(), 
        vec!["120.6".to_string(),
        "120.7".to_string(),
        "120.8".to_string(),
        "120.9".to_string()]
    )]
    #[case(
        "./file_io_test_files/laptime_file_test_collection_3.json".to_string(), 
        vec!["120.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string()]
    )]
    #[case(
        "./file_io_test_files/laptime_file_test_object_collection.json".to_string(), 
        vec!["120.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string()]
    )]
    #[case(
        "./file_io_test_files/laptime_file_test_collection_1.toml".to_string(), 
        vec!["120.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string()]
    )]
    #[case(
        "./file_io_test_files/laptime_file_test_collection_2.toml".to_string(), 
        vec!["120.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string()]
    )]
    #[case(
        "./file_io_test_files/laptime_file_test_object_collection.toml".to_string(), 
        vec!["120.6".to_string(),
            "120.7".to_string(),
            "120.8".to_string(),
            "120.9".to_string()]
    )]
    fn test_able_to_import_race_laptimes(#[case] file_name: String, #[case] laptimes: Vec<String>) {
        // Given
        let expected_race_file = RaceResultFile {
            track_name: "Default".to_string(),
            laptimes,
            ..Default::default()
        };
        let mut karting_time = KartingTime::default();

        // When
        karting_time.import_laptimes(&file_name);

        // Then
        assert!(
            std::path::Path::new(&file_name).is_file(),
            "Expected test file to exist at path: {}",
            file_name
        );
        pretty_assertions::assert_eq!(
            expected_race_file.convert_to_race_result(),
            karting_time.driver_profile.races[0]
        );
    }

    #[test]
    fn test_import_non_existent_race() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        karting_time.import_races(vec!["".to_string()]);

        // Then
        assert!(karting_time.driver_profile.races.is_empty());
    }

    #[test]
    fn test_import_races() {
        // Given
        let mut karting_time = KartingTime::default();
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
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let file_name = file_location
            .join(races[0].race_information.unique_race_identifier.to_string() + ".toml");

        // When
        upsert_races(&file_location.to_string_lossy(), &races);
        karting_time.import_races(vec![file_name.to_string_lossy().into_owned()]);

        // Then
        pretty_assertions::assert_eq!(races[0], karting_time.driver_profile.races[0]);

        // Cleanup
        let _guard = TestFileGuard::new(&file_name.to_string_lossy());
    }

    #[test]
    fn test_new_karting_time_default_state() {
        // Given
        let expected_karting_time = KartingTime::default();
        let driver_profile = DriverProfile::new(
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
        );
        let mut karting_time = KartingTime::new(driver_profile);

        // When
        karting_time.file_new();

        // Then
        pretty_assertions::assert_eq!(expected_karting_time, karting_time);
    }

    #[test]
    fn test_application_saves_then_loads() {
        // Given
        let file_name = "./file_io_test_files/karting_time_application_state.toml";
        let driver_profile = DriverProfile::new(
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
        );
        let expected = KartingTime::new(driver_profile.clone());
        let mut karting_time = KartingTime::new(driver_profile.clone());

        // When
        karting_time.load_application(file_name);

        // Then
        assert!(std::path::Path::new(file_name).is_file());
        pretty_assertions::assert_eq!(expected, karting_time);
    }
}
