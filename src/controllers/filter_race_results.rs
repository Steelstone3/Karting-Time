use crate::models::application::karting_time::KartingTime;

impl KartingTime {
    pub fn filter_race_results(&mut self) {
        match self.application_state.track_query.is_empty() {
            // track query is empty apply no filter
            true => self.application_state.filtered_races = self.driver_profile.races.clone(),
            // apply track filter
            false => {
                let query = self.application_state.track_query.to_lowercase();
                self.application_state.filtered_races = self
                    .driver_profile
                    .races
                    .iter()
                    .filter(|race| {
                        // track matches
                        race
                            .race_information
                            .track_name
                            .to_lowercase()
                            .contains(&query)
                    })
                    .cloned()
                    .collect();
            }
        }

        match self.application_state.date_query.is_empty() {
            // date query is empty apply no filter
            true => match self.application_state.track_query.is_empty() {
                // track query is empty apply no filter
                true => self.application_state.filtered_races = self.driver_profile.races.clone(),
                // apply track filter
                false => {}
            },
            // apply date filter
            false => {
                let query = self.application_state.track_query.to_lowercase();
                self.application_state.filtered_races = self
                    .driver_profile
                    .races
                    .iter()
                    .filter(|race| {
                        // date matches
                        race
                            .race_information
                            .date
                            .to_string()
                            .to_lowercase()
                            .contains(&query)
                    })
                    .cloned()
                    .collect();
            }
        }
    }
}
