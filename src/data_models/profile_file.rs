use super::race_file::RaceFile;
use crate::models::driver_profile::profile::DriverProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DriverProfileFile {
    pub name: String,
    pub races: Vec<RaceFile>,
}

impl DriverProfileFile {
    pub fn convert_to_driver_profile(&self) -> DriverProfile {
        let mut races = vec![];

        for race_file in &self.races {
            races.push(race_file.convert_to_race());
        }

        DriverProfile {
            name: self.name.to_string(),
            races,
        }
    }
}

#[cfg(test)]
mod profile_file_should {
    #[test]
    #[ignore = "reason"]
    fn convert_to_driver_profile() {}
}
