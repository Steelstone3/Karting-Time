use serde::{Deserialize, Serialize};

use crate::{models::date::Date, views::application::input_parser::parse_input_u32};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaceInformation {
    pub track_name: String,
    pub date: Date,
    pub session_id: u32,
    pub race_position: u32,
}

impl Default for RaceInformation {
    fn default() -> Self {
        Self {
            track_name: Default::default(),
            date: Default::default(),
            session_id: 1,
            race_position: 1,
        }
    }
}

impl RaceInformation {
    // TODO Test
    pub fn get_unique_race_identifier(race_information: &RaceInformation) -> String {
        format!(
            "{}_{}_{}",
            race_information.date, race_information.track_name, race_information.session_id
        )
    }

    // TODO Test
    pub fn update_session_id(&mut self, session_id: String) {
        self.session_id = parse_input_u32(session_id, 1, u32::MAX);
    }

    // TODO Test
    pub fn update_race_position(&mut self, race_position: String) {
        self.race_position = parse_input_u32(race_position, 1, u32::MAX);
    }
}

#[cfg(test)]
mod race_information_should {
    #[test]
    #[ignore = "reason"]
    fn get_unique_race_identifier() {}

    #[test]
    #[ignore = "reason"]
    fn update_session_id() {}

    #[test]
    #[ignore = "reason"]
    pub fn update_race_position() {}
}
