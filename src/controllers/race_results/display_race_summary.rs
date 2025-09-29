use std::cmp::Ordering;

use crate::models::driver::session_information::{lap::Lap, race_result::RaceResult};

impl RaceResult {
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
    use crate::models::driver::session_information::{lap::Lap, race_result::RaceResult};

    #[test]
    fn get_number_of_laps() {
        // Given
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![Lap::new(1, 23.34), Lap::new(1, 23.34), Lap::new(1, 23.34)],
        );

        // Then
        pretty_assertions::assert_eq!(3, race.get_number_of_laps())
    }

    #[test]
    fn get_no_fastest_lap() {
        // Given
        let race = RaceResult::new(Default::default(), Default::default(), Default::default());

        // Then
        pretty_assertions::assert_eq!(0.0, race.get_fastest_lap())
    }

    #[test]
    fn get_fastest_lap() {
        // Given
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![Lap::new(1, 21.67), Lap::new(1, 22.56), Lap::new(1, 20.34)],
        );

        // Then
        pretty_assertions::assert_eq!(20.34, race.get_fastest_lap())
    }

    #[test]
    fn get_no_average_lap() {
        // Given
        let race = RaceResult::new(Default::default(), Default::default(), Default::default());

        // Then
        pretty_assertions::assert_eq!(0.0, race.get_average_lap())
    }

    #[test]
    fn get_average_lap() {
        // Given
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![Lap::new(1, 21.67), Lap::new(1, 22.56), Lap::new(1, 20.34)],
        );

        // Then
        pretty_assertions::assert_eq!(20.34, race.get_average_lap())
    }
}
