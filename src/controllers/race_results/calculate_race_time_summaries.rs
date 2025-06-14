use std::{cmp::Ordering, collections::HashMap};

use crate::{
    controllers::driver_profile::time_parser::format_laptime,
    models::driver::{lap::Lap, race_result::Race},
};

impl Race {
    pub fn calculate_total_times(&self) -> HashMap<usize, f32> {
        let mut total_times = HashMap::new();
        let mut current_sum = 0.0;
        let mut current_lap = 0;

        let fastest_laps = self.order_by_fastest_lap();

        for lap in &fastest_laps {
            current_sum += lap.time;
            current_lap += 1;

            if current_lap % 5 == 0 || current_lap == fastest_laps.len() {
                total_times.insert(current_lap, current_sum);
            }
        }

        total_times
    }

    pub fn get_total_time(total_times: &HashMap<usize, f32>, key: &usize) -> String {
        match total_times.get(key) {
            Some(total_time) => format_laptime(*total_time),
            None => "N/A".to_string(),
        }
    }

    pub fn calculate_average_total_times(
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

    pub fn get_average_time(average_times: &HashMap<usize, f32>, key: &usize) -> String {
        match average_times.get(key) {
            Some(average_time) => format_laptime(*average_time),
            None => "N/A".to_string(),
        }
    }

    fn order_by_fastest_lap(&self) -> Vec<Lap> {
        let mut sorted_laps = self.laptimes.clone();
        sorted_laps.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Ordering::Greater));
        sorted_laps
    }
}

#[cfg(test)]
mod calculate_race_time_summaries_should {
    use std::collections::HashMap;

    use crate::models::driver::{lap::Lap, race_result::Race};

    #[test]
    fn calculate_total_times() {
        // Given
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 12.4,
                },
                Lap {
                    lap_number: 2,
                    time: 12.5,
                },
                Lap {
                    lap_number: 3,
                    time: 12.7,
                },
                Lap {
                    lap_number: 4,
                    time: 12.8,
                },
                Lap {
                    lap_number: 5,
                    time: 12.9,
                },
            ],
            ..Default::default()
        };

        // When
        let total_times = race.calculate_total_times();

        // Then
        let total_5_laps = *total_times.get(&5).unwrap();

        assert_eq!(63.299995, total_5_laps);
    }

    #[test]
    fn calculate_total_times_last_lap() {
        // Given
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 12.4,
                },
                Lap {
                    lap_number: 2,
                    time: 12.5,
                },
                Lap {
                    lap_number: 3,
                    time: 12.7,
                },
                Lap {
                    lap_number: 4,
                    time: 12.8,
                },
                Lap {
                    lap_number: 5,
                    time: 12.9,
                },
                Lap {
                    lap_number: 6,
                    time: 20.0,
                },
            ],
            ..Default::default()
        };

        // When
        let total_times = race.calculate_total_times();

        // Then
        let total_5_laps = *total_times.get(&5).unwrap();
        let total_6_laps = *total_times.get(&6).unwrap();

        assert_eq!(63.299995, total_5_laps);
        assert_eq!(83.299995, total_6_laps);
    }

    #[test]
    fn get_total_time() {
        // Given
        let mut total_times: HashMap<usize, f32> = HashMap::new();
        total_times.insert(5, 100.0);
        total_times.insert(10, 200.0);
        total_times.insert(15, 300.0);

        // Then
        assert_eq!("1:40.00", Race::get_total_time(&total_times, &5));
        assert_eq!("3:20.00", Race::get_total_time(&total_times, &10));
        assert_eq!("5:00.00", Race::get_total_time(&total_times, &15));
    }

    #[test]
    fn calculate_average_total_times() {
        // Given
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 45.5,
                },
                Lap {
                    lap_number: 2,
                    time: 67.9,
                },
                Lap {
                    lap_number: 3,
                    time: 50.3,
                },
                Lap {
                    lap_number: 4,
                    time: 34.6,
                },
                Lap {
                    lap_number: 5,
                    time: 34.2,
                },
            ],
            ..Default::default()
        };

        // When
        let total_times = race.calculate_total_times();
        let total_times = race.calculate_average_total_times(&total_times);

        // Then
        let average_5_laps = *total_times.get(&5).unwrap();

        assert_eq!(46.5, average_5_laps);
    }

    #[test]
    fn get_average_time() {
        // Given
        let mut average_times: HashMap<usize, f32> = HashMap::new();
        average_times.insert(5, 100.0);
        average_times.insert(10, 200.0);
        average_times.insert(15, 300.0);

        // Then
        assert_eq!("1:40.00", Race::get_total_time(&average_times, &5));
        assert_eq!("3:20.00", Race::get_total_time(&average_times, &10));
        assert_eq!("5:00.00", Race::get_total_time(&average_times, &15));
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
}
