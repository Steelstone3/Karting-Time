use crate::models::driver_profile::profile::DriverProfile;

use super::application_state::ApplicationState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct KartingTime {
    pub application_state: ApplicationState,
    pub driver_profile: DriverProfile,
}
