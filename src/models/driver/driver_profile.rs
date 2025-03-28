use crate::data_models::driver_profile_file::DriverProfileFile;
use comfy_table::{presets::ASCII_MARKDOWN, Cell, Table};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Display};

use super::race_result::Race;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriverProfile {
    pub name: String,
    pub races: Vec<Race>,
}

// TODO Test
impl Default for DriverProfile {
    fn default() -> Self {
        Self {
            name: "Racer".to_string(),
            races: Default::default(),
        }
    }
}

// TODO Test
impl Display for DriverProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        table
            // .load_preset(UTF8_FULL) // Use UTF8_FULL preset for solid lines
            .load_preset(ASCII_MARKDOWN)
            .set_header(vec![
                Cell::new("Track Name"),
                Cell::new("Date"),
                Cell::new("Session"),
                Cell::new("Race Position"),
                Cell::new("Fastest Lap"),
                Cell::new("Average Lap 5"),
                Cell::new("Average Lap 10"),
                Cell::new("Average Lap 15"),
                Cell::new("Total Lap 5"),
                Cell::new("Total Lap 10"),
                Cell::new("Total Lap 15"),
            ]);

        for race in &self.races {
            let total_times = race.calculate_total_times();
            let average_times = race.calculate_average_total_times(&total_times);
            let not_applicable_cell = Cell::new("N/A".to_string());

            let total_time_5_cell = match total_times.get(&5) {
                Some(total_time_5) => Cell::new(format!("{:.2}", total_time_5)),
                None => not_applicable_cell.clone(),
            };

            let total_time_10_cell = match total_times.get(&10) {
                Some(total_time_10) => Cell::new(format!("{:.2}", total_time_10)),
                None => not_applicable_cell.clone(),
            };

            let total_time_15_cell = match total_times.get(&15) {
                Some(total_time_15) => Cell::new(format!("{:.2}", total_time_15)),
                None => not_applicable_cell.clone(),
            };

            let average_time_5_cell = match average_times.get(&5) {
                Some(average_time_5) => Cell::new(format!("{:.2}", average_time_5)),
                None => not_applicable_cell.clone(),
            };

            let average_time_10_cell = match average_times.get(&10) {
                Some(average_time_10) => Cell::new(format!("{:.2}", average_time_10)),
                None => not_applicable_cell.clone(),
            };

            let average_time_15_cell = match average_times.get(&15) {
                Some(average_time_15) => Cell::new(format!("{:.2}", average_time_15)),
                None => not_applicable_cell.clone(),
            };

            table.add_row(vec![
                Cell::new(race.race_information.track_name.to_string()),
                Cell::new(race.race_information.date.to_string()),
                Cell::new(race.race_information.session_id.to_string()),
                Cell::new(race.race_information.race_position.to_string()),
                Cell::new(format!("{:.2}", race.get_fastest_lap())),
                average_time_5_cell,
                average_time_10_cell,
                average_time_15_cell,
                total_time_5_cell,
                total_time_10_cell,
                total_time_15_cell,
            ]);
        }

        write!(f, "{}", table)
    }
}

impl DriverProfile {
    pub fn convert_to_driver_profile_file(&self) -> DriverProfileFile {
        let mut race_files = vec![];

        for race in &self.races {
            race_files.push(race.convert_to_race_file())
        }

        DriverProfileFile {
            name: self.name.to_string(),
            races: race_files,
        }
    }

    pub fn sort_races(&mut self) {
        self.races.sort_by(|a, b| {
            b.race_information
                .date
                .cmp(&a.race_information.date)
                .then_with(|| {
                    a.race_information
                        .track_name
                        .cmp(&b.race_information.track_name)
                })
                .then_with(|| {
                    a.race_information
                        .session_id
                        .cmp(&b.race_information.session_id)
                })
        });
    }

    pub fn get_number_of_races(&self) -> u32 {
        self.races.len() as u32
    }

    pub fn get_number_of_wins(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_information.race_position == 1)
            .count() as u32
    }

    pub fn get_number_of_podiums(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_information.race_position <= 3)
            .count() as u32
    }

    pub fn get_number_of_top_fives(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_information.race_position <= 5)
            .count() as u32
    }

    pub fn get_number_of_top_tens(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_information.race_position <= 10)
            .count() as u32
    }

    pub fn get_number_of_unique_tracks(&self) -> u32 {
        let unique_tracks: HashSet<String> = self
            .races
            .iter()
            .map(|race| {
                race.race_information
                    .track_name
                    .trim()
                    .to_lowercase()
                    .clone()
            })
            .collect();

        unique_tracks.len() as u32
    }
}

#[cfg(test)]
mod driver_profile_should {
    use super::*;
    use crate::{
        data_models::race_file::RaceFile,
        models::{
            date::Date,
            driver::{lap::Lap, race_information::RaceInformation},
        },
    };

    #[test]
    fn convert_to_driver_profile_file() {
        // Given
        let expected_driver_profile_file = DriverProfileFile {
            name: "Karl Chadwick".to_string(),
            races: vec![RaceFile {
                race_information: RaceInformation {
                    track_name: "Three Ponies".to_string(),
                    date: Date {
                        day: 15,
                        month: 10,
                        year: 2024,
                    },
                    session_id: 1,
                    race_position: 2,
                },
                laptimes: vec!["50.662".to_string(), "51.877".to_string()],
            }],
        };

        let driver_profile = DriverProfile {
            name: "Karl Chadwick".to_string(),
            races: vec![Race {
                race_information: RaceInformation {
                    track_name: "Three Ponies".to_string(),
                    date: Date {
                        day: 15,
                        month: 10,
                        year: 2024,
                    },
                    session_id: 1,
                    race_position: 2,
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
            }],
        };

        // When
        let driver_profile_file = driver_profile.convert_to_driver_profile_file();

        // Then
        assert_eq!(expected_driver_profile_file, driver_profile_file)
    }

    #[test]
    fn sort_races() {
        // Given
        let race_1 = Race {
            race_information: RaceInformation {
                track_name: "Three Ponies".to_string(),
                date: Date {
                    day: 15,
                    month: 10,
                    year: 2024,
                },
                session_id: 1,
                race_position: 2,
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
        };

        let race_2 = Race {
            race_information: RaceInformation {
                track_name: "Three Ponies".to_string(),
                date: Date {
                    day: 15,
                    month: 10,
                    year: 2024,
                },
                session_id: 2,
                race_position: 1,
            },
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 50.723,
                },
                Lap {
                    lap_number: 2,
                    time: 51.956,
                },
            ],
        };

        let race_3 = Race {
            race_information: RaceInformation {
                track_name: "Trafford Stadium".to_string(),
                date: Date {
                    day: 17,
                    month: 10,
                    year: 2024,
                },
                session_id: 1,
                race_position: 1,
            },
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 30.723,
                },
                Lap {
                    lap_number: 2,
                    time: 31.956,
                },
            ],
        };

        let mut driver_profile = DriverProfile {
            name: "Karl Chadwick".to_string(),
            races: vec![race_1.clone(), race_2.clone(), race_3.clone()],
        };

        // When
        driver_profile.sort_races();

        // Then
        assert_eq!(race_3.clone(), driver_profile.races[0]);
        assert_eq!(race_1.clone(), driver_profile.races[1]);
        assert_eq!(race_2.clone(), driver_profile.races[2]);
    }

    #[test]
    fn get_number_of_races() {
        // Given
        let expected_number_of_races = 4;
        let driver_profile = DriverProfile {
            races: vec![
                Race {
                    ..Default::default()
                },
                Race {
                    ..Default::default()
                },
                Race {
                    ..Default::default()
                },
                Race {
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        // When
        let number_of_races = driver_profile.get_number_of_races();

        // Then
        assert_eq!(expected_number_of_races, number_of_races)
    }

    #[test]
    fn get_number_of_wins() {
        // Given
        let expected_number_of_wins = 2;
        let driver_profile = DriverProfile {
            races: vec![
                Race {
                    race_information: RaceInformation {
                        race_position: 1,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 5,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 1,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 3,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        // When
        let number_of_wins = driver_profile.get_number_of_wins();

        // Then
        assert_eq!(expected_number_of_wins, number_of_wins)
    }

    #[test]
    fn get_number_of_podiums() {
        // Given
        let expected_number_of_podiums = 3;
        let driver_profile = DriverProfile {
            races: vec![
                Race {
                    race_information: RaceInformation {
                        race_position: 2,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 5,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 1,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 3,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        // When
        let number_of_podiums = driver_profile.get_number_of_podiums();

        // Then
        assert_eq!(expected_number_of_podiums, number_of_podiums)
    }

    #[test]
    fn get_number_of_top_fives() {
        // Given
        let expected_number_of_top_fives = 3;
        let driver_profile = DriverProfile {
            races: vec![
                Race {
                    race_information: RaceInformation {
                        race_position: 1,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 6,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 5,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 3,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        // When
        let number_of_top_fives = driver_profile.get_number_of_top_fives();

        // Then
        assert_eq!(expected_number_of_top_fives, number_of_top_fives)
    }

    #[test]
    fn get_number_of_top_tens() {
        // Given
        let expected_number_of_top_fives = 2;
        let driver_profile = DriverProfile {
            races: vec![
                Race {
                    race_information: RaceInformation {
                        race_position: 10,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 12,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 11,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        race_position: 3,
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        // When
        let number_of_top_tens = driver_profile.get_number_of_top_tens();

        // Then
        assert_eq!(expected_number_of_top_fives, number_of_top_tens)
    }

    #[test]
    fn get_number_of_unique_tracks() {
        // Given
        let track_1 = "Three Sisters".to_string();
        let track_2 = "Llandow".to_string();
        let expected_number_of_unique_tracks = 2;
        let driver_profile = DriverProfile {
            races: vec![
                Race {
                    race_information: RaceInformation {
                        track_name: track_1.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        track_name: track_1.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        track_name: track_2.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Race {
                    race_information: RaceInformation {
                        track_name: track_2.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        };

        // When
        let number_of_unique_tracks = driver_profile.get_number_of_unique_tracks();

        // Then
        assert_eq!(expected_number_of_unique_tracks, number_of_unique_tracks)
    }
}
