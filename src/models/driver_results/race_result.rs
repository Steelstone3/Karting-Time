use super::{lap::Lap, race_information::RaceInformation};
use crate::data_models::race_file::RaceFile;
use comfy_table::{presets::ASCII_MARKDOWN, Cell, Table};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, collections::HashMap, fmt::Display};

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Race {
    pub race_information: RaceInformation,
    pub laptimes: Vec<Lap>,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        table
            // .load_preset(UTF8_FULL) // Use UTF8_FULL preset for solid lines
            .load_preset(ASCII_MARKDOWN)
            .set_header(vec![
                Cell::new("Lap"),      //.with_style(Attr::Bold),
                Cell::new("Time (s)"), //.with_style(Attr::Bold),
            ]);

        for laptime in &self.laptimes {
            table.add_row(vec![
                Cell::new(laptime.lap_number.to_string()),
                Cell::new(laptime.time.to_string()),
            ]);
        }

        write!(f, "{}", table)
    }
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

    pub fn convert_to_laps(&mut self, laptime_editor_string: String) {
        let laptimes = self.convert_string_to_laps(laptime_editor_string);

        let mut converted_laptimes = Vec::new();

        for (lap_number, &time) in laptimes.iter().enumerate() {
            converted_laptimes.push(Lap {
                lap_number: (lap_number + 1) as u32,
                time,
            });
        }

        self.laptimes = converted_laptimes;
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

    pub fn get_number_of_laps(&self) -> u32 {
        self.laptimes.len() as u32
    }

    pub fn get_fastest_lap(&self) -> f32 {
        let lap = match self
            .laptimes
            .iter()
            .min_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Ordering::Greater))
        {
            Some(lap) => lap,
            None => return 0.0,
        };

        lap.time
    }

    pub fn get_average_lap(&self) -> f32 {
        let fastest_lap_time = self.get_fastest_lap();
        let valid_average_laps: Vec<&Lap> = self
            .laptimes
            .iter()
            .filter(|lap| lap.time <= fastest_lap_time * 1.05)
            .collect();

        if valid_average_laps.is_empty() {
            0.0 // Handle the case where no laps meet the criteria
        } else {
            valid_average_laps.iter().map(|lap| lap.time).sum::<f32>()
                / valid_average_laps.len() as f32
        }
    }

    pub fn order_by_fastest_lap(&self) -> Vec<Lap> {
        let mut sorted_laps = self.laptimes.clone();
        sorted_laps.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Ordering::Greater));
        sorted_laps
    }

    pub fn convert_total_times_to_string(&self) -> String {
        let total_times = self.calculate_total_times();

        let mut total_times_string = String::new();

        // Sort the HashMap by key (lap number)
        let mut sorted_total_times: Vec<(&usize, &f32)> = total_times.iter().collect();
        sorted_total_times.sort_by(|(a, _), (b, _)| a.cmp(b)); // Sort by key (lap number)

        for (lap_number, total_time) in sorted_total_times {
            total_times_string += &format!("\nTotal Time {}: {:.2}", lap_number, total_time);
        }

        total_times_string
    }

    pub fn convert_average_total_times_to_string(&self) -> String {
        let total_times = self.calculate_total_times();
        let average_times = self.calculate_average_total_times(&total_times);

        let mut total_times_string = String::new();

        // Sort the HashMap by key (lap number)
        let mut sorted_average_times: Vec<(&usize, &f32)> = average_times.iter().collect();
        sorted_average_times.sort_by(|(a, _), (b, _)| a.cmp(b)); // Sort by key (lap number)

        for (lap_number, average_time) in sorted_average_times {
            total_times_string += &format!("\nAverage Time {}: {:.2}", lap_number, average_time);
        }

        total_times_string
    }

    pub fn convert_laps_to_string(&self) -> String {
        let mut laps = "".to_string();

        for laptime in &self.laptimes {
            laps += &format!("{}\n", laptime.time);
        }

        laps
    }

    fn calculate_total_times(&self) -> HashMap<usize, f32> {
        let mut total_times = HashMap::new();
        let mut current_sum = 0.0;
        let mut current_lap = 0;

        for lap in &self.order_by_fastest_lap() {
            current_sum += lap.time;
            current_lap += 1;

            if current_lap % 5 == 0 {
                total_times.insert(current_lap, current_sum);
            }
        }

        total_times
    }

    fn calculate_average_total_times(
        &self,
        total_times: &HashMap<usize, f32>,
    ) -> HashMap<usize, f32> {
        let mut average_times = HashMap::new();

        for total_time in total_times {
            let lap_number = *total_time.0;
            let current_total_time = *total_time.1;

            let average_time = current_total_time / lap_number as f32;

            average_times.insert(lap_number, average_time);
        }

        average_times
    }

    fn convert_laps_to_laptimes(&self) -> Vec<String> {
        let mut laps: Vec<String> = vec![];

        for laptime in &self.laptimes {
            let time = laptime.time.to_string();

            laps.push(time)
        }

        laps
    }

    fn convert_string_to_laps(&self, laptime_editor_string: String) -> Vec<f32> {
        laptime_editor_string
            .lines()
            .filter_map(|s| s.trim().parse::<f32>().ok())
            .collect()
    }
}

#[cfg(test)]
mod race_result_should {
    use rstest::rstest;

    use crate::{
        data_models::race_file::RaceFile,
        models::{
            date::Date,
            driver_results::{lap::Lap, race_information::RaceInformation},
        },
    };

    use super::Race;

    #[test]
    fn display() {
        // Given
        let expected_display =
            "| Lap | Time (s) |\n|-----|----------|\n| 1   | 12.2     |\n| 2   | 12.4     |"
                .to_string();
        let race_result = Race {
            race_information: RaceInformation {
                track_name: "Brands Hatch".to_string(),
                date: Date {
                    day: 12,
                    month: 12,
                    year: 2025,
                },
                session_id: 1,
                race_position: 1,
            },
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 12.2,
                },
                Lap {
                    lap_number: 2,
                    time: 12.4,
                },
            ],
        };

        // When
        let display = race_result.to_string();

        // Then
        assert_eq!(expected_display, display);
    }

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
    #[case("Three Sisters", "Three Sisters", "Three Sisters", 1, 2, 3,  Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 11, year: 2025 },true)]
    #[case("Three Sisters", "Trafford Park", "Brands Hatch", 1, 1, 1, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 11, year: 2025 }, true)]
    #[case("Three Sisters", "Three Sisters", "Three Sisters", 1, 1, 1, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 16, month: 11, year: 2025 }, Date{ day: 17, month: 11, year: 2025 }, true)]
    #[case("Three Sisters", "Three Sisters", "Three Sisters", 1, 1, 1, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 12, year: 2025 }, Date{ day: 15, month: 10, year: 2025 }, true)]
    #[case("Three Sisters", "Three Sisters", "Three Sisters", 1, 1, 1, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 11, year: 2024 }, Date{ day: 15, month: 11, year: 2023 }, true)]
    #[case("Three Sisters", "Three Sisters", "Three Sisters", 1, 1, 1, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 11, year: 2025 }, false)]
    #[case("", "Three Sisters", "Three Sisters", 1, 2, 3, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 11, year: 2025 }, Date{ day: 15, month: 11, year: 2025 }, false )]
    fn is_unique_identifer(
        #[case] track_name_1: &str,
        #[case] track_name_2: &str,
        #[case] track_name_3: &str,
        #[case] session_id_1: u32,
        #[case] session_id_2: u32,
        #[case] session_id_3: u32,
        #[case] date_1: Date,
        #[case] date_2: Date,
        #[case] date_3: Date,
        #[case] expected_is_unique: bool,
    ) {
        // Given
        let race_1 = Race {
            race_information: RaceInformation {
                track_name: track_name_1.to_string(),
                date: date_1,
                session_id: session_id_1,
                race_position: 2,
            },
            laptimes: vec![],
        };

        let race_2 = Race {
            race_information: RaceInformation {
                track_name: track_name_2.to_string(),
                date: date_2,
                session_id: session_id_2,
                race_position: 2,
            },
            laptimes: vec![],
        };

        let race_3 = Race {
            race_information: RaceInformation {
                track_name: track_name_3.to_string(),
                date: date_3,
                session_id: session_id_3,
                race_position: 2,
            },
            laptimes: vec![],
        };

        let races = vec![race_2, race_3];

        // When
        let is_unique = race_1.is_unique_identifer(&races);

        // Then
        assert_eq!(expected_is_unique, is_unique)
    }

    #[test]
    fn convert_to_laps() {
        // Given
        let race_editor = "53.2\n52.9\nboop";
        let mut race = Race {
            laptimes: vec![],
            ..Default::default()
        };
        let expected_laps = vec![
            Lap {
                lap_number: 1,
                time: 53.2,
            },
            Lap {
                lap_number: 2,
                time: 52.9,
            },
        ];

        // When
        race.convert_to_laps(race_editor.to_string());

        // Then
        assert_eq!(expected_laps, race.laptimes)
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

    #[test]
    fn get_number_of_laps() {
        // Given
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 23.34,
                },
                Lap {
                    lap_number: 1,
                    time: 23.34,
                },
                Lap {
                    lap_number: 1,
                    time: 23.34,
                },
            ],
            ..Default::default()
        };

        // Then
        assert_eq!(3, race.get_number_of_laps())
    }

    #[test]
    fn get_fastest_lap() {
        // Given
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 21.67,
                },
                Lap {
                    lap_number: 1,
                    time: 22.56,
                },
                Lap {
                    lap_number: 1,
                    time: 20.34,
                },
            ],
            ..Default::default()
        };

        // Then
        assert_eq!(20.34, race.get_fastest_lap())
    }

    #[test]
    fn get_average_lap() {
        // Given
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 21.67,
                },
                Lap {
                    lap_number: 1,
                    time: 22.56,
                },
                Lap {
                    lap_number: 1,
                    time: 20.34,
                },
            ],
            ..Default::default()
        };

        // Then
        assert_eq!(20.34, race.get_average_lap())
    }

    #[test]
    fn order_by_fastest_lap() {
        // Given
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 21.67,
                },
                Lap {
                    lap_number: 1,
                    time: 22.56,
                },
                Lap {
                    lap_number: 1,
                    time: 20.34,
                },
            ],
            ..Default::default()
        };

        // Then
        assert_eq!(
            vec![
                Lap {
                    lap_number: 1,
                    time: 20.34,
                },
                Lap {
                    lap_number: 1,
                    time: 21.67,
                },
                Lap {
                    lap_number: 1,
                    time: 22.56,
                },
            ],
            race.order_by_fastest_lap()
        )
    }

    #[test]
    fn convert_total_times_to_string() {
        // Given
        let expected_total_times = "\nTotal Time 5: 124.27\nTotal Time 10: 254.42".to_string();
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 25.555,
                },
                Lap {
                    lap_number: 1,
                    time: 26.657,
                },
                Lap {
                    lap_number: 1,
                    time: 24.585,
                },
                Lap {
                    lap_number: 1,
                    time: 25.475,
                },
                Lap {
                    lap_number: 1,
                    time: 24.899,
                },
                Lap {
                    lap_number: 1,
                    time: 25.345,
                },
                Lap {
                    lap_number: 1,
                    time: 26.123,
                },
                Lap {
                    lap_number: 1,
                    time: 24.879,
                },
                Lap {
                    lap_number: 1,
                    time: 26.341,
                },
                Lap {
                    lap_number: 1,
                    time: 24.563,
                },
            ],
            ..Default::default()
        };

        // When
        let total_times = race.convert_total_times_to_string();

        // Then
        assert_eq!(expected_total_times, total_times)
    }

    #[test]
    pub fn convert_average_total_times_to_string() {
        // Given
        let expected_average_laps = "\nAverage Time 5: 24.85\nAverage Time 10: 25.44".to_string();
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 25.555,
                },
                Lap {
                    lap_number: 1,
                    time: 26.657,
                },
                Lap {
                    lap_number: 1,
                    time: 24.585,
                },
                Lap {
                    lap_number: 1,
                    time: 25.475,
                },
                Lap {
                    lap_number: 1,
                    time: 24.899,
                },
                Lap {
                    lap_number: 1,
                    time: 25.345,
                },
                Lap {
                    lap_number: 1,
                    time: 26.123,
                },
                Lap {
                    lap_number: 1,
                    time: 24.879,
                },
                Lap {
                    lap_number: 1,
                    time: 26.341,
                },
                Lap {
                    lap_number: 1,
                    time: 24.563,
                },
            ],
            ..Default::default()
        };

        // When
        let average_laps = race.convert_average_total_times_to_string();

        // Then
        assert_eq!(expected_average_laps, average_laps)
    }

    #[test]
    fn convert_laps_to_string() {
        // Given
        let expected_laps =
            "25.555\n26.657\n24.585\n25.475\n24.899\n25.345\n26.123\n24.879\n26.341\n24.563\n"
                .to_string();
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 25.555,
                },
                Lap {
                    lap_number: 1,
                    time: 26.657,
                },
                Lap {
                    lap_number: 1,
                    time: 24.585,
                },
                Lap {
                    lap_number: 1,
                    time: 25.475,
                },
                Lap {
                    lap_number: 1,
                    time: 24.899,
                },
                Lap {
                    lap_number: 1,
                    time: 25.345,
                },
                Lap {
                    lap_number: 1,
                    time: 26.123,
                },
                Lap {
                    lap_number: 1,
                    time: 24.879,
                },
                Lap {
                    lap_number: 1,
                    time: 26.341,
                },
                Lap {
                    lap_number: 1,
                    time: 24.563,
                },
            ],
            ..Default::default()
        };

        // When
        let laps = race.convert_laps_to_string();

        // Then
        assert_eq!(expected_laps, laps)
    }
}
