use crate::{
    controllers::driver_profile::time_parser::format_laptime,
    models::driver::{lap::Lap, race_result::Race},
};

impl Race {
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

    pub fn convert_total_times_to_string(&self) -> String {
        let total_times = self.calculate_total_times();

        let mut total_times_string = String::new();

        // Sort the HashMap by key (lap number)
        let mut sorted_total_times: Vec<(&usize, &f32)> = total_times.iter().collect();

        // Sort by key (lap number)
        sorted_total_times.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (lap_number, total_time) in sorted_total_times {
            total_times_string += &format!(
                "\nTotal Time {}: {}",
                lap_number,
                format_laptime(*total_time)
            );
        }

        total_times_string
    }

    pub fn convert_average_total_times_to_string(&self) -> String {
        let total_times = self.calculate_total_times();
        let average_times = self.calculate_average_total_times(&total_times);

        let mut total_times_string = String::new();

        // Sort the HashMap by key (lap number)
        let mut sorted_average_times: Vec<(&usize, &f32)> = average_times.iter().collect();

        // Sort by key (lap number)
        sorted_average_times.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (lap_number, average_time) in sorted_average_times {
            total_times_string += &format!(
                "\nAverage Time {}: {}",
                lap_number,
                format_laptime(*average_time)
            );
        }

        total_times_string
    }

    fn convert_string_to_laps(&self, laptime_editor_string: String) -> Vec<f32> {
        laptime_editor_string
            .lines()
            .filter_map(|lap| {
                let trimmed_lap = lap.trim();

                if trimmed_lap.contains(':') {
                    let parts: Vec<&str> = trimmed_lap.split(':').collect();

                    let minutes = parts[0].parse::<u32>();

                    match minutes {
                        Ok(minutes) => {
                            let seconds = parts[1].parse::<f32>();

                            match seconds {
                                Ok(seconds) => Some(minutes as f32 * 60.0 + seconds),
                                Err(_) => None,
                            }
                        }
                        Err(_) => None,
                    }
                } else {
                    lap.trim().parse::<f32>().ok()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod laptime_converter_should {
    use crate::models::driver::{lap::Lap, race_result::Race};

    #[test]
    fn convert_to_laps() {
        // Given
        let race_editor = "2:45.6\n53.2\n52.9\n54\n:45.6\nboop";
        let mut race = Race {
            laptimes: vec![],
            ..Default::default()
        };
        let expected_laps = vec![
            Lap {
                lap_number: 1,
                time: 165.6,
            },
            Lap {
                lap_number: 2,
                time: 53.2,
            },
            Lap {
                lap_number: 3,
                time: 52.9,
            },
            Lap {
                lap_number: 4,
                time: 54.0,
            },
        ];

        // When
        race.convert_to_laps(race_editor.to_string());

        // Then
        assert_eq!(expected_laps, race.laptimes)
    }

    #[test]
    fn convert_total_times_to_string() {
        // Given
        let expected_total_times = "\nTotal Time 5: 2:04.27\nTotal Time 10: 4:14.42".to_string();
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
