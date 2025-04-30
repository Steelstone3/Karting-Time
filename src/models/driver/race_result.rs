use super::{lap::Lap, race_information::RaceInformation};
use crate::data_models::race_file::RaceFile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Race {
    pub race_information: RaceInformation,
    pub laptimes: Vec<Lap>,
}

impl Race {
    pub fn convert_to_race_file(&self) -> RaceFile {
        RaceFile {
            race_information: self.race_information.clone(),
            laptimes: self.convert_laps_to_laptimes(),
        }
    }

    fn convert_laps_to_laptimes(&self) -> Vec<String> {
        let mut laps: Vec<String> = vec![];

        for laptime in &self.laptimes {
            let time = laptime.time.to_string();

            laps.push(time)
        }

        laps
    }
}

#[cfg(test)]
mod race_result_should {
    use super::*;
    use crate::models::date::Date;

    #[test]
    fn convert_to_race_file() {
        // Given
        let expected_race_file = RaceFile {
            race_information: RaceInformation {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 15,
                    month: 11,
                    year: 2025,
                },
                session_id: 2,
                race_position: 7,
            },
            laptimes: vec!["54.2".to_string(), "55.6".to_string()],
        };

        let race_information = RaceInformation {
            track_name: "Three Sisters".to_string(),
            date: Date {
                day: 15,
                month: 11,
                year: 2025,
            },
            session_id: 2,
            race_position: 7,
        };
        let race = Race {
            race_information,
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 54.2,
                },
                Lap {
                    lap_number: 2,
                    time: 55.6,
                },
            ],
        };

        // When
        let race_file = race.convert_to_race_file();

        // Then
        assert_eq!(expected_race_file, race_file);
    }

    #[test]
    fn convert_laps_to_laptimes() {
        // Given
        let expected_laptime_string = vec!["10.2".to_string(), "11.8".to_string()];
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 10.2,
                },
                Lap {
                    lap_number: 2,
                    time: 11.8,
                },
            ],
            ..Default::default()
        };

        // When
        let laptime_string = race.convert_laps_to_laptimes();

        // Then
        assert_eq!(expected_laptime_string, laptime_string);
    }
}
