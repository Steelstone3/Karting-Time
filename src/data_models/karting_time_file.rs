use super::profile_file::DriverProfileFile;
use crate::models::{
    application::{application_state::ApplicationState, karting_time::KartingTime},
    driver_results::race_result::Race,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct KartingTimeFile {
    pub driver_profile: DriverProfileFile,
}

impl KartingTimeFile {
    // TODO Test
    pub fn convert_to_karting_time(&self) -> KartingTime {
        KartingTime {
            driver_profile: self.driver_profile.convert_to_driver_profile(),
            application_state: ApplicationState::default(),
            new_race: Race::default(),
        }
    }
}

#[cfg(test)]
mod karting_time_file_should {
    #[test]
    #[ignore = "reason"]
    fn convert_to_karting_time() {}
}
