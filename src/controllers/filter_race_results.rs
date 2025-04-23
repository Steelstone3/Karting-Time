use crate::models::application::karting_time::KartingTime;

impl KartingTime {
    pub fn apply_filters(&mut self) {
        self.apply_track_filter();
        self.apply_date_filter();
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
                // both filters
                false => {
                    let query = self.application_state.track_query.to_lowercase();
                    self.application_state.filtered_races = self
                        .application_state
                        .filtered_races
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
            },
        }
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
                // both filters
                false => {
                    let query = self.application_state.date_query.to_lowercase();
                    self.application_state.filtered_races = self
                        .application_state
                        .filtered_races
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
            },
        }
    }
}
