use serde::Deserialize;

#[derive(Clone, Default, Debug, Deserialize)]
pub struct AccLap {
    #[serde(rename = "driverIndex")]
    pub driver_index: u32,
    #[serde(rename = "laptime")]
    pub laptime: f32,
}

impl AccLap {
    #[allow(dead_code)]
    pub fn new(driver_index: u32, laptime: f32) -> Self {
        Self {
            driver_index,
            laptime,
        }
    }

    pub fn convert_to_laptimes(acc_laps: Vec<AccLap>) -> Vec<String> {
        acc_laps
            .into_iter()
            .map(|lap| format!("{}", lap.laptime / 1000.0))
            .collect()
    }
}

#[cfg(test)]
mod acc_lap_should {
    use crate::models::driver::session_information::acc_lap::AccLap;

    #[test]
    fn test_convert_to_laptimes_empty() {
        // Given
        let acc_laps = vec![];

        // When
        let acc_laps_by_driver = AccLap::convert_to_laptimes(acc_laps);

        // Then
        pretty_assertions::assert_eq!(0, acc_laps_by_driver.len());
    }

    #[test]
    fn test_convert_to_laptimes() {
        // Given
        let acc_laps = vec![
            AccLap::new(0, 32200.0),
            AccLap::new(1, 31200.0),
            AccLap::new(2, 33200.0),
        ];

        // When
        let acc_laps_by_driver: Vec<String> = AccLap::convert_to_laptimes(acc_laps);

        // Then
        pretty_assertions::assert_eq!(3, acc_laps_by_driver.len());
        pretty_assertions::assert_eq!("32.2".to_string(), acc_laps_by_driver[0]);
        pretty_assertions::assert_eq!("31.2".to_string(), acc_laps_by_driver[1]);
        pretty_assertions::assert_eq!("33.2".to_string(), acc_laps_by_driver[2]);
    }
}
