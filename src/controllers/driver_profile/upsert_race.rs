use crate::models::application::karting_time::KartingTime;

impl KartingTime {
    pub fn upsert_race(&mut self) {
        match self
            .application_state
            .new_race
            .is_unique_identifer(&self.driver_profile.races)
        {
            true => {
                if self
                    .application_state
                    .new_race
                    .race_information
                    .car_used
                    .is_empty()
                {
                    self.application_state.new_race.race_information.car_used = "N/A".to_string();
                }
                self.driver_profile
                    .races
                    .push(self.application_state.new_race.clone());
            }
            false => {
                self.driver_profile.races = self
                    .application_state
                    .new_race
                    .replace_existing_race(&self.driver_profile.races);
            }
        }
    }
}

#[cfg(test)]
mod upsert_race_should {
    #[test]
    #[ignore]
    fn upsert_race_no_car_used() {}

    #[test]
    #[ignore]
    fn upsert_race() {}

    #[test]
    #[ignore]
    fn upsert_race_match() {}
}
