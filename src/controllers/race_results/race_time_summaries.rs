use std::{cmp::Ordering, collections::HashMap};

use crate::models::driver::session_information::{lap::Lap, race_result::RaceResult};

impl RaceResult {
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

    pub fn get_time_by_key(times: &[(usize, String)], key_to_find: usize) -> String {
        times
            .iter()
            .find(|(k, _)| *k == key_to_find)
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| "N/A".to_string())
    }

    pub fn get_last_time(times: &[(usize, String)]) -> String {
        times
            .last()
            .map(|(_, v)| v.clone())
            .unwrap_or_else(|| "N/A".to_string())
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

    fn order_by_fastest_lap(&self) -> Vec<Lap> {
        let mut sorted_laps = self.laptimes.clone();
        sorted_laps.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Ordering::Greater));
        sorted_laps
    }
}

#[cfg(test)]
mod race_time_summaries_should {
    use crate::models::driver::session_information::{lap::Lap, race_result::RaceResult};

    #[test]
    fn calculate_total_times() {
        // Given
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![
                Lap::new(1, 12.4),
                Lap::new(2, 12.5),
                Lap::new(3, 12.7),
                Lap::new(4, 12.8),
                Lap::new(5, 12.9),
            ],
        );

        // When
        let total_times = race.calculate_total_times();

        // Then
        let total_5_laps = *total_times.get(&5).unwrap();

        pretty_assertions::assert_eq!(63.299995, total_5_laps);
    }

    #[test]
    fn calculate_total_times_last_lap() {
        // Given
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![
                Lap::new(1, 12.4),
                Lap::new(2, 12.5),
                Lap::new(3, 12.7),
                Lap::new(4, 12.8),
                Lap::new(5, 12.9),
                Lap::new(6, 20.0),
            ],
        );

        // When
        let total_times = race.calculate_total_times();

        // Then
        let total_5_laps = *total_times.get(&5).unwrap();
        let total_6_laps = *total_times.get(&6).unwrap();

        pretty_assertions::assert_eq!(63.299995, total_5_laps);
        pretty_assertions::assert_eq!(83.299995, total_6_laps);
    }

    #[test]
    fn get_time_by_key() {
        // Given
        let total_times = vec![
            (5, "1:40.00".to_string()),
            (10, "3:20.00".to_string()),
            (15, "5:00.00".to_string()),
        ];

        // Then
        pretty_assertions::assert_eq!("1:40.00", RaceResult::get_time_by_key(&total_times, 5));
        pretty_assertions::assert_eq!("3:20.00", RaceResult::get_time_by_key(&total_times, 10));
        pretty_assertions::assert_eq!("5:00.00", RaceResult::get_time_by_key(&total_times, 15));
    }

    #[test]
    fn get_last_time() {
        // Given
        let total_times = vec![
            (5, "1:40.00".to_string()),
            (10, "3:20.00".to_string()),
            (15, "5:00.00".to_string()),
        ];

        // Then
        pretty_assertions::assert_eq!("5:00.00", RaceResult::get_last_time(&total_times));
    }

    #[test]
    fn calculate_average_total_times() {
        // Given
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![
                Lap::new(1, 45.5),
                Lap::new(2, 67.9),
                Lap::new(3, 50.3),
                Lap::new(4, 34.6),
                Lap::new(5, 34.2),
            ],
        );

        // When
        let total_times = race.calculate_total_times();
        let total_times = race.calculate_average_total_times(&total_times);

        // Then
        let average_5_laps = *total_times.get(&5).unwrap();

        pretty_assertions::assert_eq!(46.5, average_5_laps);
    }

    #[test]
    fn order_by_fastest_lap() {
        // Given
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![Lap::new(1, 21.67), Lap::new(2, 22.56), Lap::new(3, 20.34)],
        );

        // Then
        pretty_assertions::assert_eq!(
            vec![Lap::new(3, 20.34), Lap::new(1, 21.67), Lap::new(2, 22.56),],
            race.order_by_fastest_lap()
        )
    }
}
