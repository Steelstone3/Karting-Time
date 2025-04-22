use crate::models::application::karting_time::KartingTime;

impl KartingTime {
    pub fn filter_race_results(&mut self) {
        if self.application_state.search_query.is_empty() {
            self.application_state.filtered_races = self.driver_profile.races.clone()
        } else {
            let query = self.application_state.search_query.to_lowercase();
            self.application_state.filtered_races = self
                .driver_profile
                .races
                .iter()
                .filter(|race| {
                    let track_name_matches = race
                        .race_information
                        .track_name
                        .to_lowercase()
                        .contains(&query);
                    let date_matches = race
                        .race_information
                        .date
                        .to_string()
                        .to_lowercase()
                        .contains(&query);
                    track_name_matches || date_matches
                })
                .cloned()
                .collect();
        }
    }
}
