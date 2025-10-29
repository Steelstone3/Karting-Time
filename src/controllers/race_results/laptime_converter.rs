use crate::{
    controllers::driver_profile::time_parser::format_laptime,
    models::driver::session_information::{lap::Lap, race_result::RaceResult},
};
use std::collections::HashMap;

impl RaceResult {
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

    pub fn convert_hash_map(hash_map: HashMap<usize, f32>) -> Vec<(usize, String)> {
        // order by key
        let mut sorted: Vec<(usize, f32)> = hash_map.into_iter().collect();
        sorted.sort_by_key(|(k, _)| *k);

        // format laptime
        sorted
            .into_iter()
            .map(|(k, v)| (k, format_laptime(v)))
            .collect()
    }

    pub fn convert_laps_to_string(&self) -> String {
        let mut laps = "".to_string();

        for laptime in &self.laptimes {
            laps += &format!("{}\n", laptime.time);
        }

        laps
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
    use crate::models::driver::session_information::{lap::Lap, race_result::RaceResult};
    use std::collections::HashMap;

    #[test]
    fn convert_to_laps() {
        // Given
        let race_editor = "2:45.6\n3:boop\n53.2\n52.9\n54\n:45.6\nboop";
        let mut race = RaceResult::default();
        let expected_laps = vec![
            Lap::new(1, 165.6),
            Lap::new(2, 53.2),
            Lap::new(3, 52.9),
            Lap::new(4, 54.0),
        ];

        // When
        race.convert_to_laps(race_editor.to_string());

        // Then
        pretty_assertions::assert_eq!(expected_laps, race.laptimes)
    }

    #[test]
    fn convert_hash_map() {
        // Given
        let expected_sorted_races = vec![
            (5, "3:50.00".to_string()),
            (10, "9:10.00".to_string()),
            (15, "12:50.00".to_string()),
        ];
        let mut races_hash_map = HashMap::new();
        races_hash_map.insert(15, 770.0);
        races_hash_map.insert(5, 230.0);
        races_hash_map.insert(10, 550.0);

        // When
        let sorted_races = RaceResult::convert_hash_map(races_hash_map);

        // Then
        pretty_assertions::assert_eq!(expected_sorted_races, sorted_races);
    }

    #[test]
    fn convert_laps_to_string() {
        // Given
        let expected_laps =
            "25.555\n26.657\n24.585\n25.475\n24.899\n25.345\n26.123\n24.879\n26.341\n24.563\n"
                .to_string();
        // TODO AH make a test fixture
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![
                Lap::new(1, 25.555),
                Lap::new(1, 26.657),
                Lap::new(1, 24.585),
                Lap::new(1, 25.475),
                Lap::new(1, 24.899),
                Lap::new(1, 25.345),
                Lap::new(1, 26.123),
                Lap::new(1, 24.879),
                Lap::new(1, 26.341),
                Lap::new(1, 24.563),
            ],
        );

        // When
        let laps = race.convert_laps_to_string();

        // Then
        pretty_assertions::assert_eq!(expected_laps, laps)
    }
}
