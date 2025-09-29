use crate::{
    data_models::driver_profile_file::DriverProfileFile,
    models::driver::{
        filter::Filter, profile_statistics::ProfileStatistics,
        session_information::race_result::RaceResult,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct DriverProfile {
    pub name: String,
    pub new_race: RaceResult,
    pub races: Vec<RaceResult>,
    pub filter: Filter,
    pub profile_statistics: ProfileStatistics,
}

impl DriverProfile {
    pub fn new(name: &str, races: Vec<RaceResult>) -> Self {
        Self {
            name: name.to_string(),
            new_race: Default::default(),
            races: races.clone(),
            filter: Filter::new_initial_state(races.clone()),
            profile_statistics: ProfileStatistics::new(races.clone()),
        }
    }

    #[allow(dead_code)]
    pub fn new_with_new_race(name: &str, new_race: RaceResult, races: Vec<RaceResult>) -> Self {
        let mut driver_profile = DriverProfile::new(name, races);
        driver_profile.new_race = new_race;

        driver_profile
    }

    pub fn convert_to_driver_profile_file(&self) -> DriverProfileFile {
        let mut race_files = vec![];

        for race in &self.races {
            race_files.push(race.convert_to_race_file())
        }

        DriverProfileFile::new(&self.name, race_files)
    }
}

impl Default for DriverProfile {
    fn default() -> Self {
        Self {
            name: "Racer".to_string(),
            new_race: Default::default(),
            races: Default::default(),
            profile_statistics: Default::default(),
            filter: Default::default(),
        }
    }
}

#[cfg(test)]
mod driver_profile_should {
    use super::*;
    use crate::{
        data_models::race_file::RaceFile,
        models::{
            date::RaceDate,
            driver::session_information::{
                lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
                race_result::RaceResult, session::Session,
            },
        },
    };

    #[test]
    fn convert_to_driver_profile_file() {
        // Given
        let expected_driver_profile_file = DriverProfileFile::new(
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

        let driver_profile = DriverProfile::new(
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

        // When
        let driver_profile_file = driver_profile.convert_to_driver_profile_file();

        // Then
        pretty_assertions::assert_eq!(expected_driver_profile_file, driver_profile_file)
    }
}
