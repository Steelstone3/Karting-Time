use crate::models::{
    date::Date,
    driver::{lap::Lap, race_information::RaceInformation, race_result::Race, session::Session},
};
use serde::{Deserialize, Serialize};
use std::f32;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaceFile {
    pub laptimes: Vec<String>,
    pub day: u32,
    pub month: u32,
    pub year: i32,
    pub track_name: String,
    pub race_position: u32,
    pub session_id: u32,
    pub session_type: Option<String>,
    pub track_conditions: Option<String>,
    pub car_used: Option<String>,
    pub notes: Option<String>,
}

impl RaceFile {
    pub fn convert_to_race(&self) -> Race {
        Race {
            race_information: self.convert_to_race_information(),
            laptimes: self.convert_laptimes_to_laps(),
            is_deleting: Default::default(),
        }
    }

    pub fn get_unique_race_information_file_identifier(race_file: &RaceFile) -> String {
        format!(
            "Date_{}_Track_{}_Session_{}",
            Date {
                day: race_file.day,
                month: race_file.month,
                year: race_file.year
            },
            race_file.track_name,
            race_file.session_id
        )
    }

    fn convert_to_race_information(&self) -> RaceInformation {
        let mut session_id = self.session_id;
        if session_id == 0 {
            session_id = 1
        }

        let session_type = match &self.session_type {
            Some(session_type) => session_type,
            None => "N/A",
        };

        let session_conditions = match &self.track_conditions {
            Some(session_conditions) => session_conditions,
            None => "N/A",
        };

        let mut race_position = self.race_position;
        if race_position == 0 {
            race_position = 1
        }

        let mut day = self.day;
        if day == 0 {
            day = 1
        }

        let mut month = self.month;
        if month == 0 {
            month = 1
        }

        let mut year = self.year;
        if year == 0 {
            year = 2000
        }

        RaceInformation {
            track_name: self.track_name.clone(),
            session: Session {
                session_id,
                session_type: session_type.to_string(),
                track_condition: session_conditions.to_string(),
                race_position,
            },
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
            date: Date { day, month, year },
        }
    }

    fn convert_laptimes_to_laps(&self) -> Vec<Lap> {
        let mut laps: Vec<Lap> = vec![];

        for (index, laptime) in self.laptimes.iter().enumerate() {
            let time = laptime.parse::<f32>();

            if let Ok(time) = time {
                laps.push(Lap {
                    lap_number: (index + 1) as u32,
                    time,
                })
            }
        }

        laps
    }
}

#[cfg(test)]
mod race_file_should {
    use super::*;
    use crate::{
        data_models::race_file::RaceFile,
        models::{date::Date, driver::race_information::RaceInformation},
    };

    #[test]
    fn convert_to_race() {
        // Given
        let expected_race = Race {
            race_information: RaceInformation {
                track_name: "Three Ponies".to_string(),
                date: Date {
                    day: 15,
                    month: 10,
                    year: 2024,
                },
                session: Session {
                    session_id: 1,
                    session_type: "N/A".to_string(),
                    track_condition: "N/A".to_string(),
                    race_position: 2,
                },
                car_used: "Kart".to_string(),
                notes: "Notes".to_string(),
            },
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 50.662,
                },
                Lap {
                    lap_number: 2,
                    time: 51.877,
                },
            ],
            ..Default::default()
        };

        let race_file = RaceFile {
            track_name: "Three Ponies".to_string(),
            day: 15,
            month: 10,
            year: 2024,
            session_id: 1,
            race_position: 2,
            car_used: Some("Kart".to_string()),
            notes: Some("Notes".to_string()),
            session_type: Some("N/A".to_string()),
            track_conditions: Some("N/A".to_string()),
            laptimes: vec!["50.662".to_string(), "51.877".to_string()],
        };

        // When
        let race = race_file.convert_to_race();

        // Then
        assert_eq!(expected_race, race)
    }
}
