use std::cmp::Ordering;

use crate::models::driver::{lap::Lap, race_result::Race};

impl Race {
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

        match valid_average_laps.is_empty() {
            true => 0.0,
            false => {
                valid_average_laps.iter().map(|lap| lap.time).sum::<f32>()
                    / valid_average_laps.len() as f32
            }
        }
    }
}

#[cfg(test)]
mod display_race_summary_should {
    use crate::models::driver::{lap::Lap, race_result::Race};

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
}
