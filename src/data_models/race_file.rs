use crate::models::driver_results::{
    lap::Lap, race_information::RaceInformation, race_result::Race,
};
use serde::{Deserialize, Serialize};
use std::f32;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct RaceFile {
    pub race_information: RaceInformation,
    pub laptimes: Vec<String>,
}

impl RaceFile {
    // TODO Test
    pub fn convert_to_race(&self) -> Race {
        Race {
            race_information: self.race_information.clone(),
            laptimes: self.convert_laptimes_to_laps(),
        }
    }

    fn convert_laptimes_to_laps(&self) -> Vec<Lap> {
        let mut laps: Vec<Lap> = vec![];

        for (index, laptime) in self.laptimes.iter().enumerate() {
            let time = laptime.parse::<f32>();

            if let Ok(time) = time {
                laps.push(Lap {
                    lap_number: (index + 1) as u32,
                    time,
                })
            }
        }

        laps
    }
}
