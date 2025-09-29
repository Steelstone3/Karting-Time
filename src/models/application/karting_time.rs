use super::application_state::ApplicationState;
use crate::{
    data_models::karting_time_file::KartingTimeFile, models::driver::driver_profile::DriverProfile,
};

#[derive(Debug, Default, PartialEq)]
pub struct KartingTime {
    pub application_state: ApplicationState,
    pub driver_profile: DriverProfile,
}

impl KartingTime {
    pub fn new(driver_profile: DriverProfile) -> Self {
        Self {
            driver_profile,
            application_state: Default::default(),
        }
    }

    pub fn convert_to_karting_time_file(&self) -> KartingTimeFile {
        KartingTimeFile::new(self.driver_profile.convert_to_driver_profile_file())
    }
}

#[cfg(test)]
mod karting_time_should {
    use crate::{
        data_models::{driver_profile_file::DriverProfileFile, race_file::RaceFile},
        models::{
            date::RaceDate,
            driver::session_information::{
                lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
                race_result::RaceResult, session::Session,
            },
        },
    };

    use super::*;

    #[test]
    fn convert_to_karting_time_file() {
        // Given
        let expected_karting_time_file = KartingTimeFile::new(DriverProfileFile::new(
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

        let karting_time = KartingTime {
            driver_profile: DriverProfile::new(
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
            ),
            application_state: Default::default(),
        };

        // When
        let karting_time_file = karting_time.convert_to_karting_time_file();

        // Then
        pretty_assertions::assert_eq!(expected_karting_time_file, karting_time_file)
    }
}
