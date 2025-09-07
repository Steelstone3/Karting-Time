use crate::{
    data_models::race_information_file::RaceInformationFile, models::date::Date,
    views::application::input_parser::parse_input_u32,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaceInformation {
    pub track_name: String,
    pub date: Date,
    pub session_id: u32,
    pub race_position: u32,
    pub car_used: String,
    pub notes: String,
}

impl Default for RaceInformation {
    fn default() -> Self {
        Self {
            track_name: Default::default(),
            date: Default::default(),
            session_id: 1,
            race_position: 1,
            car_used: "N/A".to_string(),
            notes: Default::default(),
        }
    }
}

impl RaceInformation {
    pub fn get_unique_race_information_identifier(race_information: &RaceInformation) -> String {
        format!(
            "Date_{}_Track_{}_Session_{}",
            race_information.date, race_information.track_name, race_information.session_id
        )
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = parse_input_u32(session_id, 1, u32::MAX);
    }

    pub fn set_race_position(&mut self, race_position: String) {
        self.race_position = parse_input_u32(race_position, 1, u32::MAX);
    }

    pub fn convert_to_race_information_file(&self) -> RaceInformationFile {
        let mut car_used = None;
        if !self.car_used.is_empty() {
            car_used = Some(self.car_used.clone());
        }

        let mut notes = None;
        if !self.notes.is_empty() {
            notes = Some(self.notes.clone());
        }

        RaceInformationFile {
            track_name: self.track_name.clone(),
            date: self.date.clone(),
            session_id: self.session_id,
            race_position: self.race_position,
            car_used,
            notes,
        }
    }
}

#[cfg(test)]
mod race_information_should {
    use super::RaceInformation;
    use crate::{data_models::race_information_file::RaceInformationFile, models::date::Date};

    #[test]
    fn create_race_information() {
        // Given
        let expected_race_information = RaceInformation {
            track_name: Default::default(),
            date: Default::default(),
            session_id: 1,
            race_position: 1,
            car_used: "N/A".to_string(),
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
            session_id: 1,
            race_position: 12,
            car_used: "Kart".to_string(),
            notes: Default::default(),
        };

        // When
        let race_identifier =
            RaceInformation::get_unique_race_information_identifier(&race_information);

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
            car_used: "Kart".to_string(),
            notes: Default::default(),
        };

        // When
        race_information.set_session_id(session_id.to_string());

        // Then
        assert_eq!(session_id, race_information.session_id)
    }

    #[test]
    fn update_race_position() {
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
            car_used: "Kart".to_string(),
            notes: Default::default(),
        };

        // When
        race_information.set_race_position(race_position.to_string());

        // Then
        assert_eq!(race_position, race_information.race_position)
    }

    #[test]
    fn convert_to_race_information() {
        // Given
        let expected_race_information = RaceInformationFile {
            track_name: "Three Ponies".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session_id: 1,
            race_position: 2,
            car_used: Some("Kart".to_string()),
            notes: Some("Notes".to_string()),
        };

        let race_information = RaceInformation {
            track_name: "Three Ponies".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session_id: 1,
            race_position: 2,
            car_used: "Kart".to_string(),
            notes: "Notes".to_string(),
        };

        // When
        let race_information = race_information.convert_to_race_information_file();

        // Then
        assert_eq!(expected_race_information, race_information)
    }

    #[test]
    fn convert_to_race_information_no_car_used_or_notes() {
        // Given
        let expected_race_information = RaceInformationFile {
            track_name: "Three Ponies".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session_id: 1,
            race_position: 2,
            car_used: None,
            notes: None,
        };

        let race_information = RaceInformation {
            track_name: "Three Ponies".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session_id: 1,
            race_position: 2,
            car_used: "".to_string(),
            notes: "".to_string(),
        };

        // When
        let race_information = race_information.convert_to_race_information_file();

        // Then
        assert_eq!(expected_race_information, race_information)
    }
}
