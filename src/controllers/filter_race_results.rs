use crate::models::{application::karting_time::KartingTime, driver::race_result::Race};

impl KartingTime {
    pub fn filter_race_results(&self) -> Vec<Race> {
        if self.application_state.search_query.is_empty() {
            self.driver_profile.races.clone()
        } else {
            vec![]
        }
    }
}
