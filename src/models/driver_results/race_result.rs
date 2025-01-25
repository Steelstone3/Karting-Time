use super::lap::Lap;
use crate::{models::date::Date, views::application::input_parser::parse_input_u32};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Race {
    pub track_name: String,
    pub date: Date,
    pub session_id: u32,
    pub race_position: u32,
    pub laptimes: Vec<Lap>,
}

impl Race {
    // TODO Test
    pub fn get_laptimes(&self) -> String {
        let mut laptimes = "".to_string();

        for lap in &self.laptimes {
            let lap = format!("Lap: {} | Time: {}\n", lap.lap_number, lap.time).to_string();
            laptimes += &lap;
        }

        laptimes
    }

    // TODO Test
    pub fn update_session_id(&mut self, session_id: String) {
        self.session_id = parse_input_u32(session_id, u32::MIN, u32::MAX);
    }

    // TODO Test
    pub fn update_race_position(&mut self, race_position: String) {
        self.race_position = parse_input_u32(race_position, u32::MIN, u32::MAX);
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
    fn convert_string_to_laps(&self, laptime_editor_string: String) -> Vec<f32> {
        laptime_editor_string
            .lines()
            .filter_map(|s| s.trim().parse::<f32>().ok())
            .collect()
    }
}
