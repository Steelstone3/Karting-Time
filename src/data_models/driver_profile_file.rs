use crate::models::driver::{driver_profile::DriverProfile, profile_statistics::ProfileStatistics};

use super::race_file::RaceFile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DriverProfileFile {
    pub name: String,
    pub races: Vec<RaceFile>,
    #[serde(skip)]
    pub profile_statistics: ProfileStatistics,
}

impl DriverProfileFile {
    pub fn new(name: &str, races: Vec<RaceFile>) -> Self {
        let mut driver_profile_file = Self {
            name: name.to_string(),
            races: races.clone(),
            profile_statistics: Default::default(),
        };

        driver_profile_file.profile_statistics =
            ProfileStatistics::new(RaceFile::convert_to_race_results(races.clone()));

        driver_profile_file
    }

    pub fn convert_to_driver_profile(&self) -> DriverProfile {
        let mut races = vec![];

        for race_file in &self.races {
            races.push(race_file.convert_to_race_result());
        }

        DriverProfile::new(&self.name, races)
    }
}

#[cfg(test)]
mod profile_file_should {
    use crate::models::{
        date::RaceDate,
        driver::session_information::{
            lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
            race_result::RaceResult, session::Session,
        },
    };

    use super::*;

    #[test]
    fn convert_to_driver_profile() {
        // Given
        let expected_driver_profile = DriverProfile::new(
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
        );

        let driver_profile_file = DriverProfileFile::new(
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
        );

        // When
        let driver_profile = driver_profile_file.convert_to_driver_profile();

        // Then
        pretty_assertions::assert_eq!(expected_driver_profile, driver_profile)
    }
}
