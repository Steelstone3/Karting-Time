use crate::controllers::file::html_converter::convert_to_html;
use crate::data_models::karting_time_file::KartingTimeFile;
use crate::data_models::race_result_file::RaceResultFile;
use crate::models::date::RaceDate;
use crate::models::driver::driver_profile::DriverProfile;
use crate::models::driver::session_information::acc_lap::AccLap;
use crate::models::driver::session_information::acc_session_data::AccSessionData;
use crate::models::driver::session_information::race_metadata::RaceMetadata;
use crate::models::driver::session_information::race_result::RaceResult;
use crate::models::driver::session_information::session::Session;
use maud::Markup;
use std::fs::File;
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
    let markup: Markup = convert_to_html(driver_profile);

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

pub fn read_acc_laptimes_file(file_name: &str) -> Vec<Option<RaceResultFile>> {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return vec![];
    }

    let mut race_result_files = vec![];

    let session_data: AccSessionData = serde_json::from_str(&contents).unwrap_or_default();

    let grouped = session_data.group_laps_by_driver();

    for (session_index, (driver_index, laps)) in (1001..).zip(grouped) {
        race_result_files.push(Some(RaceResultFile::new(
            &session_data.track_name,
            AccLap::convert_to_laptimes(laps),
            RaceMetadata::new(
                &session_data.session_type,
                Default::default(),
                Default::default(),
                Default::default(),
                "Imported from ACC",
            ),
            Session::new(
                session_index,
                session_data.calculate_race_position(driver_index),
            ),
            RaceDate::today(),
        )));
    }

    race_result_files
}

pub fn read_laptimes_file(file_name: &str) -> Option<RaceResultFile> {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return None;
    }

    let laptimes: Vec<String> = contents
        .lines()
        .map(|line| line.trim().trim_end_matches(',').to_string())
        .map(|line| {
            line.chars()
                .filter(|char| char.is_ascii_digit() || *char == '.' || *char == ':')
                .collect::<String>()
        })
        .filter(|line| line.chars().any(|c| c.is_ascii_digit()))
        .collect();

    if laptimes.is_empty() {
        return None;
    }

    Some(RaceResultFile::new_from_laptime_file(laptimes))
}

pub fn read_race_file(file_name: &str) -> Option<RaceResultFile> {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return None;
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

pub fn read_application_state(file_name: &str) -> Option<KartingTimeFile> {
    let contents = get_file_contents(file_name);

    if contents.is_empty() {
        return None;
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
        controllers::file::test_file_guard::test_file_guard::TestFileGuard,
        data_models::driver_profile_file::DriverProfileFile,
        models::{
            date::RaceDate,
            driver::session_information::{
                lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
                session::Session,
            },
        },
    };
    use rstest::rstest;

    #[test]
    fn test_upsert_races_failed_to_create_file() {
        // Given
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
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let file_name = file_location.join(
            "nonexistent_dir/".to_string()
                + &races[0].race_information.unique_race_identifier
                + ".toml",
        );

        // When
        upsert_races(&file_location.to_string_lossy(), &races);

        // Then
        assert!(!std::path::Path::new(&file_name).exists());

        // Cleanup
        let _guard = TestFileGuard::new(&file_name.to_string_lossy());
    }

    #[test]
    fn test_upsert_races() {
        // Given
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
        let file_identifier = format!("{}.toml", races[0].race_information.unique_race_identifier);
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let file_path = file_location.join(&file_identifier);

        // When
        upsert_races(&file_location.to_string_lossy(), &races);

        // Then
        assert!(std::path::Path::new(&file_path).is_file());
    }

    #[test]
    fn test_upsert_races_html_failed_to_create_file() {
        // Given
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
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let file_name = file_location.join(driver_profile.clone().name + "/nonexistent_dir/.html");

        // When
        upsert_html_races(&file_location.to_string_lossy(), &driver_profile);

        // Then
        assert!(!std::path::Path::new(&file_name).exists());

        // Cleanup
        let _guard = TestFileGuard::new(&file_name.to_string_lossy());
    }

    #[test]
    fn test_upsert_races_html() {
        // Given
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
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let file_name = file_location.join(driver_profile.clone().name + ".html");

        // When
        upsert_html_races(&file_location.to_string_lossy(), &driver_profile.clone());

        // Then
        assert!(std::path::Path::new(&file_name).is_file());

        // Cleanup
        let _guard = TestFileGuard::new(&file_name.to_string_lossy());
    }

    #[test]
    fn test_read_non_existent_acc_laptime_file() {
        // When
        let race_file = read_acc_laptimes_file("");

        // Then
        assert!(race_file.is_empty());
    }

    #[test]
    fn test_read_one_player_acc_laptime_file() {
        // Given
        let expected_race_file = RaceResultFile::new(
            "silverstone",
            vec![
                "122.505".to_string(),
                "122.147".to_string(),
                "121.615".to_string(),
                "121.1".to_string(),
                "121.935".to_string(),
                "123.527".to_string(),
                "122.215".to_string(),
                "121.702".to_string(),
                "122.18".to_string(),
                "121.297".to_string(),
                "120.785".to_string(),
                "120.522".to_string(),
            ],
            RaceMetadata::new("FP", "N/A", "", "", "Imported from ACC"),
            Session::new(1001, 1),
            RaceDate::today(),
        );

        let file_name = "./file_io_test_files/acc_file_1.json";

        // When
        let race_files = read_acc_laptimes_file(file_name);

        // Then
        assert!(
            std::path::Path::new(&file_name).is_file(),
            "Expected test file to exist at path: {}",
            file_name
        );

        pretty_assertions::assert_eq!(1, race_files.len());
        pretty_assertions::assert_eq!(Some(expected_race_file), race_files[0].clone());
    }

    #[test]
    fn test_read_multiple_player_acc_laptime_file() {
        // Given
        let track_name = "silverstone";
        let race_meta_data = RaceMetadata::new("FP", "N/A", "", "", "Imported from ACC");
        let race_date = RaceDate::today();

        let expected_race_file_1 = RaceResultFile::new(
            track_name,
            vec![
                "122.147".to_string(),
                "121.1".to_string(),
                "122.215".to_string(),
                "122.18".to_string(),
                "121.297".to_string(),
            ],
            race_meta_data.clone(),
            Session::new(1001, 3),
            race_date.clone(),
        );
        let expected_race_file_2 = RaceResultFile::new(
            track_name,
            vec![
                "122.505".to_string(),
                "121.615".to_string(),
                "121.702".to_string(),
                "120.785".to_string(),
            ],
            race_meta_data.clone(),
            Session::new(1002, 2),
            race_date.clone(),
        );
        let expected_race_file_3 = RaceResultFile::new(
            track_name,
            vec![
                "121.935".to_string(),
                "123.527".to_string(),
                "120.522".to_string(),
            ],
            race_meta_data.clone(),
            Session::new(1003, 1),
            race_date.clone(),
        );

        let file_name = "./file_io_test_files/acc_file_2.json";

        // When
        let race_files = read_acc_laptimes_file(file_name);

        // Then
        assert!(
            std::path::Path::new(&file_name).is_file(),
            "Expected test file to exist at path: {}",
            file_name
        );
        pretty_assertions::assert_eq!(3, race_files.len());
        pretty_assertions::assert_eq!(Some(expected_race_file_1), race_files[0].clone());
        pretty_assertions::assert_eq!(Some(expected_race_file_2), race_files[1].clone());
        pretty_assertions::assert_eq!(Some(expected_race_file_3), race_files[2].clone());
    }

    #[test]
    fn test_read_non_existent_laptime_file() {
        // When
        let race_file = read_laptimes_file("");

        // Then
        assert!(race_file.is_none());
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
    fn test_read_laptime_file(#[case] file_name: String, #[case] laptimes: Vec<String>) {
        // Given
        let expected_race_file = RaceResultFile {
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
        pretty_assertions::assert_eq!(Some(expected_race_file), race_file);
    }

    #[test]
    fn test_read_laptime_file_with_no_laptimes() {
        // Given
        let file_name = "./file_io_test_files/laptime_file_test_empty.json";

        // When
        let race_file = read_laptimes_file(file_name);

        // Then
        assert!(
            std::path::Path::new(&file_name).is_file(),
            "Expected test file to exist at path: {}",
            file_name
        );
        pretty_assertions::assert_eq!(None, race_file);
    }

    #[test]
    fn test_read_non_existent_race_file() {
        // When
        let race_file = read_race_file("");

        // Then
        assert!(race_file.is_none());
    }

    #[test]
    fn test_read_race_file() {
        // Given
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
        let expected_race_file = RaceResultFile::new(
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
        let race_file = read_race_file(&file_name.to_string_lossy());

        // Then
        pretty_assertions::assert_eq!(Some(expected_race_file), race_file);

        // Cleanup
        let _guard = TestFileGuard::new(&file_name.to_string_lossy());
    }

    #[test]
    fn test_read_race_file_no_session_type_or_session_conditions_or_car_used_or_notes() {
        // Given
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
        let expected_race_file = RaceResultFile::new(
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
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();

        // When
        upsert_races(&file_location.to_string_lossy(), &races);
        let file_name = file_location
            .join(races[0].race_information.unique_race_identifier.to_string() + ".toml");
        let race_file = read_race_file(&file_name.to_string_lossy());

        // Then
        pretty_assertions::assert_eq!(Some(expected_race_file), race_file);

        // Cleanup
        let _guard = TestFileGuard::new(&file_name.to_string_lossy());
    }

    #[test]
    fn test_read_application_state_empty() {
        // Given
        let file_name = "non_existent_file.toml";

        // When

        let karting_time = read_application_state(file_name);

        // Then
        assert!(karting_time.is_none());

        // Cleanup
        let _guard = TestFileGuard::new(file_name);
    }

    #[test]
    fn test_upsert_application_state_failed_to_create_file() {
        // Given
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let file_name =
            file_location.join("nonexistent_dir/".to_string() + "karting_time_test_file_1.toml");
        let karting_time_file = KartingTimeFile::default();

        // When
        upsert_application_state(&file_name.to_string_lossy(), &karting_time_file);

        // Then
        assert!(!std::path::Path::new(&file_name).exists());

        // Cleanup
        let _guard = TestFileGuard::new(&file_name.to_string_lossy());
    }

    #[test]
    fn test_upsert_application_state_to_file() {
        // Given
        let file_name = "karting_time_test_file_1.toml";
        let karting_time_file = KartingTimeFile::default();

        // When

        upsert_application_state(file_name, &karting_time_file);

        // Then
        assert!(std::path::Path::new(file_name).is_file());

        // Cleanup
        let _guard = TestFileGuard::new(file_name);
    }

    #[test]
    fn test_read_application_state_from_file() {
        // Given
        let file_name = "./file_io_test_files/karting_time_application_state.toml";

        let driver_profile_file = DriverProfileFile::new(
            "Jack Jackson",
            vec![
                RaceResultFile::new(
                    "Three Sisters",
                    vec!["50.4".to_string(), "55.5".to_string()],
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    Session::new(1, 1),
                    RaceDate::new(12, 12, 2025),
                ),
                RaceResultFile::new(
                    "Trafford Park",
                    vec!["56.8".to_string(), "58.7".to_string()],
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    Session::new(2, 3),
                    RaceDate::new(15, 1, 2024),
                ),
            ],
        );
        let expected_karting_time_file = KartingTimeFile::new(driver_profile_file.clone());

        // When
        let karting_time_file = read_application_state(file_name);

        // Then
        assert!(karting_time_file.is_some());
        pretty_assertions::assert_eq!(Some(expected_karting_time_file), karting_time_file);
    }
}
