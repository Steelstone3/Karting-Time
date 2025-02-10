use super::application_state::ApplicationState;
use crate::{
    data_models::karting_time_file::KartingTimeFile,
    models::{driver_profile::profile::DriverProfile, driver_results::race_result::Race},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct KartingTime {
    #[serde(skip)]
    pub application_state: ApplicationState,
    pub driver_profile: DriverProfile,
    #[serde(skip)]
    pub new_race: Race,
}

impl KartingTime {
    // TODO Test
    pub fn convert_to_karting_time_file(&self) -> KartingTimeFile {
        KartingTimeFile {
            driver_profile: self.driver_profile.convert_to_driver_profile_file(),
        }
    }
}
