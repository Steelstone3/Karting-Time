use std::{cmp::Ordering, collections::HashMap, fmt::Display};

use super::lap::Lap;
use crate::{models::date::Date, views::application::input_parser::parse_input_u32};
use comfy_table::{presets::ASCII_MARKDOWN, Cell, Table};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Race {
    pub track_name: String,
    pub date: Date,
    pub session_id: u32,
    pub race_position: u32,
    pub laptimes: Vec<Lap>,
}

impl Default for Race {
    fn default() -> Self {
        Self {
            session_id: 1,
            race_position: 1,
            track_name: Default::default(),
            date: Default::default(),
            laptimes: Default::default(),
        }
    }
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
    // TODO Test
    pub fn get_unique_race_identifier(race: &Race) -> String {
        format!("{}_{}_{}", race.session_id, race.track_name, race.date)
    }

    // TODO Test
    pub fn is_unique_identifer(&self, races: &Vec<Race>) -> bool {
        for race in races {
            if Race::get_unique_race_identifier(self) == Race::get_unique_race_identifier(race) {
                return false;
            }
        }

        true
    }

    // TODO Test
    pub fn update_session_id(&mut self, session_id: String) {
        self.session_id = parse_input_u32(session_id, 1, u32::MAX);
    }

    // TODO Test
    pub fn update_race_position(&mut self, race_position: String) {
        self.race_position = parse_input_u32(race_position, 1, u32::MAX);
    }

    // TODO Test
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

    // TODO Test
    pub fn replace_existing_race(&self, races: &[Race]) -> Vec<Race> {
        let mut updated_races = races.to_owned();

        for i in 0..updated_races.len() {
            if Race::get_unique_race_identifier(self)
                == Race::get_unique_race_identifier(&updated_races[i])
            {
                updated_races[i] = self.clone();
                return updated_races;
            }
        }

        updated_races
    }

    // TODO Test
    pub fn get_number_of_laps(&self) -> u32 {
        self.laptimes.len() as u32
    }

    // TODO Test
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

    // TODO Test
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

    // TODO Test
    pub fn order_by_fastest_lap(&self) -> Vec<Lap> {
        let mut sorted_laps = self.laptimes.clone();
        sorted_laps.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap_or(Ordering::Greater));
        sorted_laps
    }

    // TODO Test
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

    // TODO Test
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

    //TODO Test
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

    fn convert_string_to_laps(&self, laptime_editor_string: String) -> Vec<f32> {
        laptime_editor_string
            .lines()
            .filter_map(|s| s.trim().parse::<f32>().ok())
            .collect()
    }
}

#[cfg(test)]
mod race_result_should {
    use crate::models::{date::Date, driver_results::lap::Lap};

    use super::Race;

    #[test]
    fn display() {
        // Given
        let expected_display =
            "| Lap | Time (s) |\n|-----|----------|\n| 1   | 12.2     |\n| 2   | 12.4     |"
                .to_string();
        let race_result = Race {
            track_name: "Brands Hatch".to_string(),
            date: Date {
                day: 12,
                month: 12,
                year: 2025,
            },
            session_id: 1,
            race_position: 1,
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
    #[ignore = "reason"]
    fn get_a_unique_race_identifier() {}

    #[test]
    #[ignore = "reason"]
    fn is_unique_identifer() {}

    #[test]
    #[ignore = "reason"]
    fn update_session_id() {}

    #[test]
    #[ignore = "reason"]
    fn update_race_position() {}

    #[test]
    #[ignore = "reason"]
    fn convert_to_laps() {}

    #[test]
    #[ignore = "reason"]
    fn replace_existing_race() {}

    #[test]
    #[ignore = "reason"]
    fn get_number_of_laps() {}

    #[test]
    #[ignore = "reason"]
    fn get_fastest_lap() {}

    #[test]
    #[ignore = "reason"]
    fn get_average_lap() {}

    #[test]
    #[ignore = "reason"]
    fn order_by_fastest_lap() {}

    #[test]
    #[ignore = "reason"]
    fn convert_total_times_to_string() {}

    #[test]
    #[ignore = "reason"]
    fn convert_average_total_times_to_string() {}

    #[test]
    #[ignore = "reason"]
    fn convert_laps_to_string() {}
}
