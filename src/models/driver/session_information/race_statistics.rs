use crate::{
    controllers::driver_profile::time_parser::{format_laptime, format_laptimes},
    models::driver::session_information::race_result::RaceResult,
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RaceStatistics {
    pub average_times_table: Vec<(usize, String)>,
    pub total_times_table: Vec<(usize, String)>,
    pub formatted_laps: Vec<String>,
    pub fastest_lap: String,
    pub number_of_laps: u32,
    pub average_105_lap: String,
}

impl RaceStatistics {
    pub fn new(race_result: &RaceResult) -> Self {
        let total_times = race_result.calculate_total_times();
        let average_times = race_result.calculate_average_total_times(&total_times);

        Self {
            average_times_table: RaceResult::convert_hash_map(average_times),
            total_times_table: RaceResult::convert_hash_map(total_times),
            fastest_lap: format_laptime(race_result.get_fastest_lap()),
            formatted_laps: format_laptimes(race_result.laptimes.clone()),
            number_of_laps: race_result.get_number_of_laps(),
            average_105_lap: format_laptime(race_result.get_average_lap()),
        }
    }
}
