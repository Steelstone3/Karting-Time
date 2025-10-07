use super::driver_profile_file::DriverProfileFile;
use crate::models::application::karting_time::KartingTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct KartingTimeFile {
    pub driver_profile_file: DriverProfileFile,
}

impl KartingTimeFile {
    pub fn new(driver_profile_file: DriverProfileFile) -> Self {
        Self {
            driver_profile_file,
        }
    }

    pub fn convert_to_karting_time(&self) -> KartingTime {
        KartingTime::new(self.driver_profile_file.convert_to_driver_profile())
    }
}

#[cfg(test)]
mod karting_time_file_should {
    use super::*;
    use crate::{
        data_models::race_file::RaceFile,
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

    #[test]
    fn convert_to_karting_time() {
        // Given
        let expected_karting_time = KartingTime::new(DriverProfile::new(
            "Karl Chadwick",
            vec![RaceResult::new(
                RaceInformation::new(
                    "Three Ponies",
                    RaceDate::new(15, 10, 2024),
                    Session::new(1, 2),
                ),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    "Kart",
                    "Championship",
                    "Notes",
                ),
                vec![Lap::new(1, 50.662), Lap::new(2, 51.877)],
            )],
        ));

        let karting_time_file = KartingTimeFile::new(DriverProfileFile::new(
            "Karl Chadwick",
            vec![RaceFile::new(
                "Three Ponies",
                vec!["50.662".to_string(), "51.877".to_string()],
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    "Kart",
                    "Championship",
                    "Notes",
                ),
                Session::new(1, 2),
                RaceDate::new(15, 10, 2024),
            )],
        ));

        // When
        let karting_time = karting_time_file.convert_to_karting_time();

        // Then
        pretty_assertions::assert_eq!(expected_karting_time, karting_time)
    }
}
