use crate::models::{
    date::RaceDate,
    driver::session_information::{
        lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
        race_result::RaceResult, race_statistics::RaceStatistics, session::Session,
    },
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
    pub session_id: u32,
    pub race_position: u32,
    pub session_type: Option<String>,
    pub track_conditions: Option<String>,
    pub car_used: Option<String>,
    pub championship: Option<String>,
    pub notes: Option<String>,
    #[serde(skip)]
    pub race_statistics: RaceStatistics,
}

impl RaceFile {
    #[allow(dead_code)]
    pub fn new(
        track_name: &str,
        laptimes: Vec<String>,
        race_metadata: RaceMetadata,
        session: Session,
        date: RaceDate,
    ) -> Self {
        let mut session_type: Option<String> = None;
        if !race_metadata.session_type.is_empty() {
            session_type = Some(race_metadata.session_type)
        }

        let mut track_conditions: Option<String> = None;
        if !race_metadata.track_conditions.is_empty() {
            track_conditions = Some(race_metadata.track_conditions)
        }

        let mut car_used: Option<String> = None;
        if !race_metadata.car_used.is_empty() {
            car_used = Some(race_metadata.car_used)
        }

        let mut championship: Option<String> = None;
        if !race_metadata.championship.is_empty() {
            championship = Some(race_metadata.championship)
        }

        let mut notes: Option<String> = None;
        if !race_metadata.notes.is_empty() {
            notes = Some(race_metadata.notes)
        }

        let mut race_file = Self {
            laptimes,
            day: date.day,
            month: date.month,
            year: date.year,
            track_name: track_name.to_string(),
            session_id: session.session_id,
            race_position: session.race_position,
            session_type,
            track_conditions,
            car_used,
            championship,
            notes,
            race_statistics: Default::default(),
        };

        race_file.with_race_statistics();

        race_file
    }

    pub fn with_race_statistics(&mut self) {
        let race_result = self.convert_to_race_result();

        self.race_statistics = RaceStatistics::new(&race_result);
    }

    pub fn convert_to_race_results(race_files: Vec<RaceFile>) -> Vec<RaceResult> {
        let mut race_results = vec![];

        for race_file in race_files {
            race_results.push(race_file.convert_to_race_result());
        }

        race_results
    }

    pub fn convert_to_race_result(&self) -> RaceResult {
        RaceResult::new(
            self.convert_to_race_information(),
            self.convert_to_race_metadata(),
            self.convert_laptimes_to_laps(),
        )
    }

    fn convert_to_race_information(&self) -> RaceInformation {
        let mut session_id = self.session_id;
        if session_id == 0 {
            session_id = 1
        }

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

        RaceInformation::new(
            &self.track_name.clone(),
            RaceDate { day, month, year },
            Session {
                session_id,
                race_position,
            },
        )
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

    fn convert_to_race_metadata(&self) -> RaceMetadata {
        let session_type = match &self.session_type {
            Some(session_type) => session_type,
            None => "N/A",
        }
        .to_string();

        let track_conditions = match &self.track_conditions {
            Some(session_conditions) => session_conditions,
            None => "N/A",
        }
        .to_string();

        let car_used = match &self.car_used {
            Some(car_used) => car_used,
            None => "N/A",
        }
        .to_string();

        let championship = match &self.championship {
            Some(championship) => championship,
            None => "",
        }
        .to_string();

        let notes = match &self.notes {
            Some(notes) => notes,
            None => "",
        }
        .to_string();

        RaceMetadata {
            session_type,
            track_conditions,
            car_used,
            notes,
            championship,
        }
    }
}

#[cfg(test)]
mod race_file_should {
    use super::*;
    use crate::{data_models::race_file::RaceFile, models::date::RaceDate};

    #[test]
    fn convert_to_race() {
        // Given
        let expected_race = RaceResult::new(
            RaceInformation::new(
                "Three Ponies",
                RaceDate::new(15, 10, 2024),
                Session::new(1, 2),
            ),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                "Kart",
                "Championship",
                "Notes",
            ),
            vec![Lap::new(1, 50.662), Lap::new(2, 51.877)],
        );

        let race_file = RaceFile::new(
            "Three Ponies",
            vec!["50.662".to_string(), "51.877".to_string()],
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                "Kart",
                "Championship",
                "Notes",
            ),
            Session::new(1, 2),
            RaceDate::new(15, 10, 2024),
        );

        // When
        let race = race_file.convert_to_race_result();

        // Then
        pretty_assertions::assert_eq!(expected_race, race)
    }
}
