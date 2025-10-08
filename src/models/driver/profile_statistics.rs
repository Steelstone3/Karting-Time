use crate::{
    controllers::driver_profile::profile_statistics::{
        get_filtered_number_of_podiums, get_filtered_number_of_races,
        get_filtered_number_of_top_fives, get_filtered_number_of_top_tens,
        get_filtered_number_of_unique_cars, get_filtered_number_of_unique_tracks,
        get_filtered_number_of_wins,
    },
    models::driver::session_information::race_result::RaceResult,
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProfileStatistics {
    pub races: u32,
    pub wins: u32,
    pub podiums: u32,
    pub top_5: u32,
    pub top_10: u32,
    pub unique_tracks: u32,
    pub unique_cars: u32,
}

impl ProfileStatistics {
    pub fn new(filtered_races: Vec<RaceResult>) -> Self {
        Self {
            races: get_filtered_number_of_races(&filtered_races),
            wins: get_filtered_number_of_wins(&filtered_races),
            podiums: get_filtered_number_of_podiums(&filtered_races),
            top_5: get_filtered_number_of_top_fives(&filtered_races),
            top_10: get_filtered_number_of_top_tens(&filtered_races),
            unique_tracks: get_filtered_number_of_unique_tracks(&filtered_races),
            unique_cars: get_filtered_number_of_unique_cars(&filtered_races),
        }
    }
}
