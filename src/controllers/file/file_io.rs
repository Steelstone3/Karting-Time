use crate::controllers::file::html_converter::convert_to_html;
use crate::data_models::karting_time_file::KartingTimeFile;
use crate::data_models::race_file::RaceFile;
use crate::models::driver::driver_profile::DriverProfile;
use crate::models::driver::session_information::race_result::RaceResult;
use maud::Markup;
use std::fs::{self, File};
use std::io::{Read, Write};

const FILE_ERROR: &str = "failed to create file";

pub fn upsert_races(folder_location: &str, races: &Vec<RaceResult>) {
    for race in races {
        let race_file = race.convert_to_race_file();

        let file_name = format!(
            "{}/{}.toml",
            folder_location, race.race_information.unique_race_identifier
        );

        let mut file = match File::create(file_name) {
            Ok(file) => file,
            Err(_) => {
                println!("{FILE_ERROR}");
                return;
            }
        };

        let toml = toml::to_string_pretty(&race_file).unwrap_or_default();

        write!(file, "{toml}").unwrap_or_default()
    }
}

pub fn upsert_html_races(folder_location: &str, driver_profile: &DriverProfile) {
    let markup: Markup = convert_to_html(&driver_profile.convert_to_driver_profile_file());

    let file_name = format!("{}/{}.html", folder_location, &driver_profile.name);

    let mut file = match File::create(file_name) {
        Ok(file) => file,
        Err(_) => {
            println!("{FILE_ERROR}");
            return;
        }
    };

    write!(file, "{}", markup.into_string()).unwrap_or_default()
}

pub fn read_laptimes_file(file_name: &str) -> RaceFile {
    let contents = get_file_contents(file_name);

    let normalised_file_name = file_name.to_string().to_lowercase();

    if contents.is_empty() {
        return Default::default();
    }

    if normalised_file_name.ends_with(".txt") {
        read_laptime_list_file_strategy(file_name)
    } else if normalised_file_name.ends_with(".md") {
        read_laptime_list_file_strategy(file_name)
    } else if normalised_file_name.ends_with(".csv") {
        read_laptime_list_file_strategy(file_name)
    } else if normalised_file_name.ends_with(".json") {
        read_laptime_list_file_strategy(file_name)
    } else if normalised_file_name.ends_with(".toml") {
        read_laptime_list_file_strategy(file_name)
    } else {
        RaceFile::default()
    }
}

fn read_laptime_list_file_strategy(file_name: &str) -> RaceFile {
    let contents = match fs::read_to_string(file_name) {
        Ok(contents) => contents,
        Err(_) => return RaceFile::default(),
    };

    let laptimes = contents
        .lines()
        .map(|line| line.trim().trim_end_matches(',').to_string())
        .map(|line| {
            line.chars()
                .filter(|char| char.is_ascii_digit() || *char == '.' || *char == ':')
                .collect::<String>()
        })
        .filter(|line| line.chars().any(|c| c.is_ascii_digit()))
        .collect();

    RaceFile::new_from_laptime_file(laptimes)
}

pub fn read_race_file(file_name: &str) -> RaceFile {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return Default::default();
    }

    toml::from_str(&contents).unwrap_or_default()
}

pub fn upsert_application_state(file_path: &str, karting_time: &KartingTimeFile) {
    let mut file = match File::create(file_path) {
        Ok(file) => file,
        Err(_) => {
            println!("{FILE_ERROR}");
            return;
        }
    };

    let toml = toml::to_string_pretty(&karting_time).unwrap_or_default();

    write!(file, "{toml}").unwrap_or_default()
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
        controllers::file::test_file_guard::TestFileGuard,
        models::{
            date::RaceDate,
            driver::session_information::{
                lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
                session::Session,
            },
        },
    };
    use rstest::rstest;
    use std::fs;

    #[test]
    fn upsert_races_test_failed_to_create_file() {
        // Given
        let file_location = "/";
        // TODO make a test fixture
        let races = vec![RaceResult::new(
            RaceInformation::new(
                "Three Sisters",
                RaceDate::new(1, 1, 2025),
                Session::new(1, 1),
            ),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            Default::default(),
        )];
        let mut path = std::env::temp_dir();
        path.push(
            "nonexistent_dir/".to_string()
                + &races[0].race_information.unique_race_identifier
                + ".toml",
        );
        let file_name = path.to_str().unwrap();

        // When
        let _guard = TestFileGuard::new(&file_name);

        upsert_races(file_location, &races);

        // Then
        assert!(fs::metadata(&file_name).is_err());
    }

    #[test]
    fn upsert_races_test() {
        // Given
        let file_location = ".";
        // TODO Make a test fixture
        let races = vec![RaceResult::new(
            RaceInformation::new(
                "Three Sisters",
                RaceDate::new(1, 1, 2025),
                Session::new(1, 1),
            ),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            Default::default(),
        )];

        vec![RaceResult::new(
            RaceInformation::new(
                "Three Sisters",
                RaceDate::new(1, 1, 2025),
                Session::new(1, Default::default()),
            ),
            RaceMetadata::default(),
            Default::default(),
        )];

        // When
        upsert_races(file_location, &races);

        // Then
        let file_name =
            "./".to_string() + &races[0].race_information.unique_race_identifier + ".toml";
        let _guard = TestFileGuard::new(&file_name);

        assert!(fs::metadata(&file_name).is_ok());
        assert_ne!(fs::metadata(&file_name).unwrap().len(), 0);
    }

    #[test]
    fn upsert_races_html_test_failed_to_create_file() {
        // Given
        let file_location = "/";
        // TODO Make a test fixture
        let driver_profile = DriverProfile::new(
            "Obi Wan Kenobi",
            vec![RaceResult::new(
                RaceInformation::new(
                    "Three Sisters",
                    RaceDate::new(14, 12, 2025),
                    Session::new(1, 1),
                ),
                RaceMetadata::new("Race", "Wet", "Ferrari", "Ferrari Challenge", "No notes"),
                vec![Lap::new(1, 20.0)],
            )],
        );
        let mut path = std::env::temp_dir();
        path.push("nonexistent_dir/".to_string() + &format!("/{}.html", driver_profile.name));
        let file_name = path.to_str().unwrap();

        // When
        let _guard = TestFileGuard::new(&file_name);

        upsert_html_races(file_location, &driver_profile);

        // Then
        assert!(fs::metadata(&file_name).is_err());
    }

    #[test]
    fn upsert_races_html_test() {
        // Given
        let file_location = ".";
        // TODO make a test fixture
        let driver_profile = DriverProfile::new(
            "Obi Wan Kenobi",
            vec![RaceResult::new(
                RaceInformation::new(
                    "Three Sisters",
                    RaceDate::new(14, 12, 2025),
                    Session::new(1, 1),
                ),
                RaceMetadata::new("Race", "Wet", "Ferrari", "Ferrari Challenge", "No notes"),
                vec![Lap::new(1, 20.0)],
            )],
        );
        let file_name = format!("./{}.html", driver_profile.name);

        // When
        let _guard = TestFileGuard::new(&file_name);

        upsert_html_races(file_location, &driver_profile);

        // Then
        assert!(fs::metadata(&file_name).is_ok());
        assert_ne!(fs::metadata(&file_name).unwrap().len(), 0);
    }

    #[test]
    fn read_non_existent_laptime_file_test() {
        // Given
        let expected_race_file = RaceFile::default();

        // When
        let race_file = read_laptimes_file("");

        // Then
        pretty_assertions::assert_eq!(expected_race_file, race_file);
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
    fn read_laptime_file_test(#[case] file_name: String, #[case] laptimes: Vec<String>) {
        // Given
        let expected_race_file = RaceFile {
            track_name: "Default".to_string(),
            laptimes,
            ..Default::default()
        };

        // When
        let race_file = read_laptimes_file(&file_name);

        // Then
        assert!(
            std::path::Path::new(&file_name).is_file(),
            "Expected test file to exist at path: {}",
            file_name
        );
        pretty_assertions::assert_eq!(expected_race_file, race_file);
    }

    #[test]
    fn read_non_existent_race_file_test() {
        // Given
        let expected_race_file = RaceFile::default();

        // When
        let race_file = read_race_file("");

        // Then
        pretty_assertions::assert_eq!(expected_race_file, race_file);
    }

    #[test]
    fn read_race_file_test() {
        // Given
        let file_location = "./";
        let races = vec![RaceResult::new(
            RaceInformation::new(
                "Three Sisters",
                RaceDate::new(17, 5, 2026),
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
        let expected_race_file = RaceFile::new(
            "Three Sisters",
            Default::default(),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                "Kart",
                "Championship",
                "Notes",
            ),
            Session::new(1, 1),
            RaceDate::new(17, 5, 2026),
        );

        // When
        upsert_races(file_location, &races);
        let file_name =
            "./".to_string() + &races[0].race_information.unique_race_identifier + ".toml";

        let _guard = TestFileGuard::new(&file_name);

        let race_file = read_race_file(&file_name);

        // Then
        pretty_assertions::assert_eq!(expected_race_file, race_file);
    }

    #[test]
    fn read_race_file_test_no_session_type_or_session_conditions_or_car_used_or_notes() {
        // Given
        let file_location = "./";
        let races = vec![RaceResult::new(
            RaceInformation::new(
                "Three Sisters",
                RaceDate::new(27, 5, 2026),
                Session::new(1, 1),
            ),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            Default::default(),
        )];

        let expected_race_file = RaceFile::new(
            "Three Sisters",
            Default::default(),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
                Default::default(),
            ),
            Session::new(1, 1),
            RaceDate::new(27, 5, 2026),
        );

        // When
        upsert_races(file_location, &races);
        let file_name =
            "./".to_string() + &races[0].race_information.unique_race_identifier + ".toml";

        let _guard = TestFileGuard::new(&file_name);

        let race_file = read_race_file(&file_name);

        // Then
        pretty_assertions::assert_eq!(expected_race_file, race_file);
    }

    #[test]
    fn read_application_state_empty() {
        // Given
        let file_name = "non_existent_file.toml";
        let expected_application = KartingTimeFile::default();

        // When
        let _guard = TestFileGuard::new(file_name);

        let karting_time = read_application_state(file_name);

        // Then
        pretty_assertions::assert_eq!(expected_application, karting_time);
    }

    #[test]
    fn upsert_application_state_failed_to_create_file() {
        // Given
        let mut path = std::env::temp_dir();
        path.push("nonexistent_dir/".to_string() + "karting_time_test_file_1.toml");
        let file_name = path.to_str().unwrap();
        let karting_time_file = KartingTimeFile::default();

        // When
        let _guard = TestFileGuard::new(file_name);

        upsert_application_state(file_name, &karting_time_file);

        // Then
        assert!(fs::metadata(file_name).is_err());
    }

    #[test]
    fn upsert_application_state_to_file() {
        // Given
        let file_name = "karting_time_test_file_1.toml";
        let karting_time_file = KartingTimeFile::default();

        // When
        let _guard = TestFileGuard::new(file_name);

        upsert_application_state(file_name, &karting_time_file);

        // Then
        assert!(fs::metadata(file_name).is_ok());
        assert_ne!(fs::metadata(file_name).unwrap().len(), 0);
    }

    #[test]
    fn acceptance_test_read_application_state_from_file() {
        // Given
        let file_name = "karting_time_test_file_2.toml";
        let expected_karting_time = KartingTimeFile::default();

        // When
        let _guard = TestFileGuard::new(file_name);

        upsert_application_state(file_name, &expected_karting_time);
        let karting_time = read_application_state(file_name);

        // Then
        pretty_assertions::assert_eq!(expected_karting_time, karting_time);
    }
}
