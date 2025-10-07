use crate::{
    controllers::driver_profile::time_parser::{format_laptime, format_laptimes},
    models::driver::session_information::race_result::RaceResult,
};

#[derive(Debug, Clone, PartialEq)]
pub struct RaceStatistics {
    pub average_5: String,
    pub average_10: String,
    pub average_15: String,
    pub total_5: String,
    pub total_10: String,
    pub total_15: String,
    pub total_time: String,
    pub average_times_summary: String,
    pub average_times_table: Vec<(usize, f32)>,
    pub total_times_summary: String,
    pub total_times_table: Vec<(usize, f32)>,
    pub formatted_laps: Vec<String>,
    pub fastest_lap: String,
    pub number_of_laps: u32,
    pub average_105_lap: String,
}

impl Default for RaceStatistics {
    fn default() -> Self {
        Self {
            average_5: "N/A".to_string(),
            average_10: "N/A".to_string(),
            average_15: "N/A".to_string(),
            total_5: "N/A".to_string(),
            total_10: "N/A".to_string(),
            total_15: "N/A".to_string(),
            total_time: "N/A".to_string(),
            average_times_summary: Default::default(),
            average_times_table: Default::default(),
            total_times_summary: Default::default(),
            total_times_table: Default::default(),
            formatted_laps: Default::default(),
            fastest_lap: Default::default(),
            number_of_laps: Default::default(),
            average_105_lap: Default::default(),
        }
    }
}

impl RaceStatistics {
    pub fn new(race_result: &RaceResult) -> Self {
        let total_times = race_result.calculate_total_times();
        let average_times = race_result.calculate_average_total_times(&total_times);

        let average_5 = RaceResult::get_average_time_by_key(&average_times, &5);
        let average_10 = RaceResult::get_average_time_by_key(&average_times, &10);
        let average_15 = RaceResult::get_average_time_by_key(&average_times, &15);

        let total_5 = RaceResult::get_total_time_by_key(&total_times, &5);
        let total_10 = RaceResult::get_total_time_by_key(&total_times, &10);
        let total_15 = RaceResult::get_total_time_by_key(&total_times, &15);
        let total_time = RaceResult::get_last_total_time(&total_times);

        Self {
            fastest_lap: format_laptime(race_result.get_fastest_lap()),
            average_5,
            average_10,
            average_15,
            total_5,
            total_10,
            total_15,
            total_time,
            average_times_summary: race_result.convert_average_total_times_to_string(),
            average_times_table: RaceResult::convert_hash_map(total_times),
            total_times_summary: race_result.convert_total_times_to_string(),
            total_times_table: RaceResult::convert_hash_map(average_times),
            formatted_laps: format_laptimes(race_result.laptimes.clone()),
            number_of_laps: race_result.get_number_of_laps(),
            average_105_lap: format_laptime(race_result.get_average_lap()),
        }
    }
}
