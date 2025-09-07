use crate::models::{date::Date, driver::race_information::RaceInformation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaceInformationFile {
    pub track_name: String,
    pub date: Date,
    // TODO AH Wrap into a session
    pub session_id: u32,
    pub session_type: Option<String>,
    pub session_conditions: Option<String>,
    pub race_position: u32,
    // Up to here
    pub car_used: Option<String>,
    pub notes: Option<String>,
}

impl RaceInformationFile {
    pub fn convert_to_race_information(&self) -> RaceInformation {
        let mut session_id = self.session_id;
        if session_id == 0 {
            session_id = 1
        }

        let session_type = match &self.session_type {
            Some(session_type) => session_type,
            None => "N/A",
        };

        let session_conditions = match &self.session_conditions {
            Some(session_conditions) => session_conditions,
            None => "N/A",
        };

        let mut race_position = self.race_position;
        if race_position == 0 {
            race_position = 1
        }

        RaceInformation {
            track_name: self.track_name.clone(),
            date: self.date.clone(),
            session_id,
            session_type: session_type.to_string(),
            track_conditions: session_conditions.to_string(),
            race_position,
            car_used: match &self.car_used {
                Some(car_used) => car_used,
                None => "N/A",
            }
            .to_string(),
            notes: match &self.notes {
                Some(notes) => notes,
                None => "",
            }
            .to_string(),
        }
    }

    pub fn get_unique_race_information_file_identifier(
        race_information_file: &RaceInformationFile,
    ) -> String {
        format!(
            "Date_{}_Track_{}_Session_{}",
            race_information_file.date,
            race_information_file.track_name,
            race_information_file.session_id
        )
    }
}

#[cfg(test)]
mod race_information_file_should {
    use super::*;
    use crate::models::{date::Date, driver::race_information::RaceInformation};

    #[test]
    fn convert_to_race_information() {
        // Given
        let expected_race_information = RaceInformation {
            track_name: "Three Ponies".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session_id: 1,
            session_type: "N/A".to_string(),
            track_conditions: "N/A".to_string(),
            race_position: 2,
            car_used: "Kart".to_string(),
            notes: "Notes".to_string(),
        };

        let race_information = RaceInformationFile {
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
            session_type: Some("N/A".to_string()),
            session_conditions: Some("N/A".to_string()),
        };

        // When
        let race_information = race_information.convert_to_race_information();

        // Then
        assert_eq!(expected_race_information, race_information)
    }

    #[test]
    fn convert_to_race_information_no_car_used_or_notes() {
        // Given
        let expected_race_information = RaceInformation {
            track_name: "Three Ponies".to_string(),
            date: Date {
                day: 15,
                month: 10,
                year: 2024,
            },
            session_id: 1,
            session_type: "N/A".to_string(),
            track_conditions: "N/A".to_string(),
            race_position: 2,
            car_used: "N/A".to_string(),
            notes: "".to_string(),
        };

        let race_information = RaceInformationFile {
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
            session_type: None,
            session_conditions: None,
        };

        // When
        let race_information = race_information.convert_to_race_information();

        // Then
        assert_eq!(expected_race_information, race_information)
    }
}
