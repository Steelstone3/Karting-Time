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

    pub fn is_unique_identifer(&self, races: &Vec<Race>) -> bool {
        for race in races {
            if RaceInformation::get_unique_race_identifier(&self.race_information)
                == RaceInformation::get_unique_race_identifier(&race.race_information)
                || self.race_information.track_name.is_empty()
            {
                return false;
            }
        }

        true
    }

    pub fn replace_existing_race(&self, races: &[Race]) -> Vec<Race> {
        let mut updated_races = races.to_owned();

        for i in 0..updated_races.len() {
            if RaceInformation::get_unique_race_identifier(&self.race_information)
                == RaceInformation::get_unique_race_identifier(&updated_races[i].race_information)
            {
                updated_races[i] = self.clone();
                return updated_races;
            }
        }

        updated_races
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
    use rstest::rstest;

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

    #[rstest]
    #[case(Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() },Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() },Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() }, false)]
    #[case(Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() },Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 16, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() },Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 17, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() }, true)]
    #[case(Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() },Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 2, race_position: 1 }, ..Default::default() },Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 3, race_position: 1 }, ..Default::default() }, true)]
    #[case(Race{ race_information: RaceInformation { track_name: "Three Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() },Race{ race_information: RaceInformation { track_name: "Four Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() },Race{ race_information: RaceInformation { track_name: "Five Sisters".to_string(), date: Date{ day: 15, month: 11, year: 2025 }, session_id: 1, race_position: 1 }, ..Default::default() }, true)]
    fn is_unique_identifer(
        #[case] race_1: Race,
        #[case] race_2: Race,
        #[case] race_3: Race,
        #[case] expected_is_unique: bool,
    ) {
        // Given
        let races = vec![race_2, race_3];

        // When
        let is_unique = race_1.is_unique_identifer(&races);

        // Then
        assert_eq!(expected_is_unique, is_unique)
    }

    #[test]
    fn replace_existing_race() {
        // Given
        let updated_race = Race {
            race_information: RaceInformation {
                track_name: "Trafford Park".to_string(),
                date: Date {
                    day: 15,
                    month: 11,
                    year: 2025,
                },
                session_id: 1,
                race_position: 5,
            },
            laptimes: vec![Lap {
                lap_number: 1,
                time: 10.5,
            }],
        };

        let existing_races = vec![
            Race {
                race_information: RaceInformation {
                    track_name: "Trafford Park".to_string(),
                    date: Date {
                        day: 15,
                        month: 11,
                        year: 2025,
                    },
                    session_id: 1,
                    race_position: 3,
                },
                laptimes: vec![Lap {
                    lap_number: 1,
                    time: 9.5,
                }],
            },
            Race {
                race_information: RaceInformation {
                    track_name: "Trafford Park".to_string(),
                    date: Date {
                        day: 15,
                        month: 11,
                        year: 2025,
                    },
                    session_id: 2,
                    race_position: 5,
                },
                laptimes: vec![Lap {
                    lap_number: 1,
                    time: 10.5,
                }],
            },
        ];

        let expected_updated_existing_races = vec![
            Race {
                race_information: RaceInformation {
                    track_name: "Trafford Park".to_string(),
                    date: Date {
                        day: 15,
                        month: 11,
                        year: 2025,
                    },
                    session_id: 1,
                    race_position: 5,
                },
                laptimes: vec![Lap {
                    lap_number: 1,
                    time: 10.5,
                }],
            },
            Race {
                race_information: RaceInformation {
                    track_name: "Trafford Park".to_string(),
                    date: Date {
                        day: 15,
                        month: 11,
                        year: 2025,
                    },
                    session_id: 2,
                    race_position: 5,
                },
                laptimes: vec![Lap {
                    lap_number: 1,
                    time: 10.5,
                }],
            },
        ];

        // When
        let updated_existing_races = updated_race.replace_existing_race(&existing_races);

        // Then
        assert_eq!(expected_updated_existing_races, updated_existing_races)
    }
}
