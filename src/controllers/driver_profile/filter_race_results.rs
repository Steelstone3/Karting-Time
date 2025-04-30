use crate::models::application::karting_time::KartingTime;

impl KartingTime {
    pub fn apply_filters(&mut self) {
        match self.application_state.is_filter_visible {
            true => {
                self.apply_track_filter();
                self.apply_date_filter();
            }
            false => self.application_state.filtered_races = self.driver_profile.races.clone(),
        }
    }

    fn apply_track_filter(&mut self) {
        match self.application_state.track_query.is_empty() {
            // track query is empty apply no filter
            true => match self.application_state.date_query.is_empty() {
                // date query is empty apply no filter
                true => self.application_state.filtered_races = self.driver_profile.races.clone(),
                // keep date filter
                false => {}
            },
            // apply track filter
            false => match self.application_state.date_query.is_empty() {
                // only track filter
                true => {
                    self.apply_track_filter_only();
                }
                // both filters
                false => {
                    self.apply_both_filters();
                }
            },
        }
    }

    fn apply_track_filter_only(&mut self) {
        let query = self.application_state.track_query.to_lowercase();
        self.application_state.filtered_races = self
            .driver_profile
            .races
            .iter()
            .filter(|race| {
                // track matches
                race.race_information
                    .track_name
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect();
    }

    fn apply_date_filter(&mut self) {
        match self.application_state.date_query.is_empty() {
            // date query is empty apply no filter
            true => match self.application_state.track_query.is_empty() {
                // track query is empty apply no filter
                true => self.application_state.filtered_races = self.driver_profile.races.clone(),
                // keep track filter
                false => {}
            },
            // apply date filter
            false => match self.application_state.track_query.is_empty() {
                // only date filter
                true => {
                    self.apply_date_filter_only();
                }
                // both filters
                false => {
                    self.apply_both_filters();
                }
            },
        }
    }

    fn apply_date_filter_only(&mut self) {
        let query = self.application_state.date_query.to_lowercase();
        self.application_state.filtered_races = self
            .driver_profile
            .races
            .iter()
            .filter(|race| {
                // date matches
                race.race_information
                    .date
                    .to_string()
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect();
    }

    fn apply_both_filters(&mut self) {
        let track_query = self.application_state.track_query.to_lowercase();
        let date_query = self.application_state.date_query.to_lowercase();
        self.application_state.filtered_races = self
            .driver_profile
            .races
            .iter()
            .filter(|race| {
                // track matches
                let is_track_filtered = race
                    .race_information
                    .track_name
                    .to_lowercase()
                    .contains(&track_query);
                // date matches
                let is_date_filtered = race
                    .race_information
                    .date
                    .to_string()
                    .to_lowercase()
                    .contains(&date_query);

                is_track_filtered && is_date_filtered
            })
            .cloned()
            .collect();
    }
}

#[cfg(test)]
mod filter_race_results_should {}
