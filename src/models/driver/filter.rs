use crate::models::driver::session_information::race_result::RaceResult;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Filter {
    pub is_filter_visible: bool,
    pub track_query: String,
    pub date_query: String,
    pub car_used_query: String,
    pub championship_query: String,
    pub session_type_query: String,
    pub filtered_races: Vec<RaceResult>,
}

impl Filter {
    #[allow(dead_code)]
    pub fn new(
        is_filter_visible: bool,
        track_query: &str,
        date_query: &str,
        car_used_query: &str,
        championship_query: &str,
        session_type_query: &str,
        filtered_races: Vec<RaceResult>,
    ) -> Self {
        Self {
            is_filter_visible,
            track_query: track_query.to_string(),
            date_query: date_query.to_string(),
            car_used_query: car_used_query.to_string(),
            championship_query: championship_query.to_string(),
            session_type_query: session_type_query.to_string(),
            filtered_races,
        }
    }

    pub fn new_initial_state(filtered_races: Vec<RaceResult>) -> Self {
        Self {
            filtered_races,
            ..Default::default()
        }
    }
}
