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
    pub fn get_unique_race_identifier(race_information: &RaceInformation) -> String {
        format!(
            "{}_{}_{}",
            race_information.date, race_information.track_name, race_information.session_id
        )
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = parse_input_u32(session_id, 1, u32::MAX);
    }

    pub fn set_race_position(&mut self, race_position: String) {
        self.race_position = parse_input_u32(race_position, 1, u32::MAX);
    }
}

#[cfg(test)]
mod race_information_should {
    use super::RaceInformation;
    use crate::models::date::Date;

    #[test]
    fn create_race_information() {
        // Given
        let expected_race_information = RaceInformation {
            track_name: Default::default(),
            date: Default::default(),
            session_id: 1,
            race_position: 1,
        };

        // Then
        assert_eq!(expected_race_information, RaceInformation::default())
    }

    #[test]
    fn get_unique_race_identifier() {
        // Given
        let expected_race_identifer = "2024-10-15_Three Sisters_1".to_string();
        let race_information = RaceInformation {
            track_name: "Three Sisters".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session_id: 1,
            race_position: 12,
        };

        // When
        let race_identifier = RaceInformation::get_unique_race_identifier(&race_information);

        // Then
        assert_eq!(expected_race_identifer, race_identifier)
    }

    #[test]
    fn update_session_id() {
        // Given
        let session_id = 5;
        let mut race_information = RaceInformation {
            track_name: "Three Sisters".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session_id: 1,
            race_position: 12,
        };

        // When
        race_information.set_session_id(session_id.to_string());

        // Then
        assert_eq!(session_id, race_information.session_id)
    }

    #[test]
    pub fn update_race_position() {
        // Given
        let race_position = 5;
        let mut race_information = RaceInformation {
            track_name: "Three Sisters".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session_id: 1,
            race_position: 12,
        };

        // When
        race_information.set_race_position(race_position.to_string());

        // Then
        assert_eq!(race_position, race_information.race_position)
    }
}
