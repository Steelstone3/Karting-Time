use crate::models::driver::{lap::Lap, race_result::Race};

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
}
