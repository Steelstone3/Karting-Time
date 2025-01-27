use crate::models::{driver_profile::profile::DriverProfile, driver_results::race_result::Race};

use super::application_state::ApplicationState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct KartingTime {
    #[serde(skip)]
    pub application_state: ApplicationState,
    pub driver_profile: DriverProfile,
    #[serde(skip)]
    pub new_race: Race,
}
