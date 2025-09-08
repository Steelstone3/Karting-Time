use crate::models::{date::Date, driver::session::Session};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaceInformation {
    pub track_name: String,
    pub date: Date,
    pub session: Session,
    pub car_used: String,
    pub championship: String,
    pub notes: String,
}

impl Default for RaceInformation {
    fn default() -> Self {
        Self {
            track_name: Default::default(),
            car_used: "N/A".to_string(),
            date: Default::default(),
            session: Session::default(),
            championship: Default::default(),
            notes: Default::default(),
        }
    }
}

impl RaceInformation {
    pub fn get_unique_race_information_identifier(race_information: &RaceInformation) -> String {
        format!(
            "Date_{}_Track_{}_Session_{}",
            race_information.date, race_information.track_name, race_information.session.session_id
        )
    }
}

#[cfg(test)]
mod race_information_should {
    use super::RaceInformation;
    use crate::models::{date::Date, driver::session::Session};

    #[test]
    fn create_race_information() {
        // Given
        let expected_race_information = RaceInformation {
            track_name: Default::default(),
            date: Default::default(),
            session: Default::default(),
            car_used: "N/A".to_string(),
            championship: Default::default(),
            notes: Default::default(),
        };

        // Then
        assert_eq!(expected_race_information, RaceInformation::default())
    }

    #[test]
    fn get_unique_race_identifier() {
        // Given
        let expected_race_identifer = "Date_2024-10-15_Track_Three Sisters_Session_1".to_string();
        let race_information = RaceInformation {
            track_name: "Three Sisters".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session: Session {
                session_id: 1,
                session_type: "N/A".to_string(),
                track_condition: "N/A".to_string(),
                race_position: 12,
            },
            car_used: "Kart".to_string(),
            championship: "Championship".to_string(),
            notes: Default::default(),
        };

        // When
        let race_identifier =
            RaceInformation::get_unique_race_information_identifier(&race_information);

        // Then
        assert_eq!(expected_race_identifer, race_identifier)
    }
}
