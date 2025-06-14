use crate::models::application::karting_time::KartingTime;

impl KartingTime {
    pub fn update_filtering(&mut self) {
        match self.application_state.is_filter_visible {
            true => self.apply_filters(),
            false => self.application_state.filtered_races = self.driver_profile.races.clone(),
        }
    }

    fn apply_filters(&mut self) {
        self.application_state.filtered_races = self.driver_profile.races.clone();

        if !self.all_filters_empty() {
            self.all_filters();
        } else if self.application_state.track_query.is_empty() {
            self.date_filter();
            self.car_used_filter();
        } else if self.application_state.date_query.is_empty() {
            self.track_name_filter();
            self.car_used_filter();
        } else if self.application_state.car_used_query.is_empty() {
            self.track_name_filter();
            self.date_filter();
        } else if self.application_state.track_query.is_empty()
            && self.application_state.date_query.is_empty()
        {
            self.car_used_filter();
        } else if self.application_state.track_query.is_empty()
            && self.application_state.car_used_query.is_empty()
        {
            self.date_filter();
        } else if self.application_state.date_query.is_empty()
            && self.application_state.car_used_query.is_empty()
        {
            self.track_name_filter();
        }
    }

    fn track_name_filter(&mut self) {
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

    fn date_filter(&mut self) {
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
            .collect()
    }

    fn car_used_filter(&mut self) {
        let query = self.application_state.car_used_query.to_lowercase();
        self.application_state.filtered_races = self
            .application_state
            .filtered_races
            .iter()
            .filter(|race| {
                // car used matches
                race.race_information
                    .car_used
                    .to_string()
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect()
    }

    fn all_filters_empty(&self) -> bool {
        self.application_state.track_query.is_empty()
            && self.application_state.date_query.is_empty()
            && self.application_state.car_used_query.is_empty()
    }

    fn all_filters(&mut self) {
        self.track_name_filter();
        self.date_filter();
        self.car_used_filter();
    }
}

#[cfg(test)]
mod filter_race_results_should {
    use crate::models::application::application_state::ApplicationState;
    use crate::models::date::Date;
    use crate::models::driver::race_information::RaceInformation;
    use crate::models::driver::{lap::Lap, race_result::Race};
    use crate::models::{
        application::karting_time::KartingTime, driver::driver_profile::DriverProfile,
    };
    use rstest::rstest;

    #[rstest]
    #[case(false, "", "", "", vec![ Race { race_information: RaceInformation {track_name:"Silverstone".to_string(),date:Date{day:21,month:12,year:2025},session_id:1,race_position:1, car_used: "Kart Type 1".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 25.6 }], ..Default::default() }, Race { race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:12,month:8,year:2024},session_id:1,race_position:2, car_used: "Kart Type 2".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 28.2 }], ..Default::default() } ])]
    #[case(true, "", "", "", vec![ Race { race_information: RaceInformation {track_name:"Silverstone".to_string(),date:Date{day:21,month:12,year:2025},session_id:1,race_position:1, car_used: "Kart Type 1".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 25.6 }], ..Default::default() }, Race { race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:12,month:8,year:2024},session_id:1,race_position:2, car_used: "Kart Type 2".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 28.2 }], ..Default::default() } ])]
    #[case(true, "Silverstone", "2025-12-21", "Kart Type 1", vec![ Race { race_information: RaceInformation {track_name:"Silverstone".to_string(),date:Date{day:21,month:12,year:2025},session_id:1,race_position:1, car_used: "Kart Type 1".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 25.6 }], ..Default::default() } ])]
    #[case(true, "Silverstone", "2025-12-21", "", vec![ Race { race_information: RaceInformation {track_name:"Silverstone".to_string(),date:Date{day:21,month:12,year:2025},session_id:1,race_position:1, car_used: "Kart Type 1".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 25.6 }], ..Default::default() } ])]
    #[case(true, "Silverstone", "", "Kart Type 1", vec![ Race { race_information: RaceInformation {track_name:"Silverstone".to_string(),date:Date{day:21,month:12,year:2025},session_id:1,race_position:1, car_used: "Kart Type 1".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 25.6 }], ..Default::default() } ])]
    #[case(true, "", "2025-12-21", "Kart Type 1", vec![ Race { race_information: RaceInformation {track_name:"Silverstone".to_string(),date:Date{day:21,month:12,year:2025},session_id:1,race_position:1, car_used: "Kart Type 1".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 25.6 }], ..Default::default() } ])]
    #[case(true, "Silverstone", "", "", vec![ Race { race_information: RaceInformation {track_name:"Silverstone".to_string(),date:Date{day:21,month:12,year:2025},session_id:1,race_position:1, car_used: "Kart Type 1".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 25.6 }], ..Default::default() } ])]
    #[case(true, "", "2025-12-21", "", vec![ Race { race_information: RaceInformation {track_name:"Silverstone".to_string(),date:Date{day:21,month:12,year:2025},session_id:1,race_position:1, car_used: "Kart Type 1".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 25.6 }], ..Default::default() } ])]
    #[case(true, "", "", "Kart Type 1", vec![ Race { race_information: RaceInformation {track_name:"Silverstone".to_string(),date:Date{day:21,month:12,year:2025},session_id:1,race_position:1, car_used: "Kart Type 1".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 25.6 }], ..Default::default() } ])]
    #[case(true, "Three Sisters", "2024-8-12", "Kart Type 2", vec![ Race { race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:12,month:8,year:2024},session_id:1,race_position:2, car_used: "Kart Type 2".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 28.2 }], ..Default::default() } ])]
    #[case(true, "Three Sisters", "2024-8-12", "", vec![ Race { race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:12,month:8,year:2024},session_id:1,race_position:2, car_used: "Kart Type 2".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 28.2 }], ..Default::default() } ])]
    #[case(true, "Three Sisters", "", "Kart Type 2", vec![ Race { race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:12,month:8,year:2024},session_id:1,race_position:2, car_used: "Kart Type 2".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 28.2 }], ..Default::default() } ])]
    #[case(true, "", "2024-8-12", "Kart Type 2", vec![ Race { race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:12,month:8,year:2024},session_id:1,race_position:2, car_used: "Kart Type 2".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 28.2 }], ..Default::default() } ])]
    #[case(true, "Three Sisters", "", "", vec![ Race { race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:12,month:8,year:2024},session_id:1,race_position:2, car_used: "Kart Type 2".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 28.2 }], ..Default::default() } ])]
    #[case(true, "", "2024-8-12", "", vec![ Race { race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:12,month:8,year:2024},session_id:1,race_position:2, car_used: "Kart Type 2".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 28.2 }], ..Default::default() } ])]
    #[case(true, "", "", "Kart Type 2", vec![ Race { race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:12,month:8,year:2024},session_id:1,race_position:2, car_used: "Kart Type 2".to_string() }, laptimes: vec![Lap { lap_number: 1, time: 28.2 }], ..Default::default() } ])]
    fn update_filtering(
        #[case] is_filter_visible: bool,
        #[case] track_query: String,
        #[case] date_query: String,
        #[case] car_used_query: String,
        #[case] expected_races: Vec<Race>,
    ) {
        // Given
        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                is_filter_visible,
                track_query,
                date_query,
                car_used_query,
                ..Default::default()
            },
            driver_profile: DriverProfile {
                name: Default::default(),
                races: vec![
                    Race {
                        race_information: RaceInformation {
                            track_name: "Silverstone".to_string(),
                            date: Date {
                                day: 21,
                                month: 12,
                                year: 2025,
                            },
                            session_id: 1,
                            race_position: 1,
                            car_used: "Kart Type 1".to_string(),
                        },
                        laptimes: vec![Lap {
                            lap_number: 1,
                            time: 25.6,
                        }],
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            track_name: "Three Sisters".to_string(),
                            date: Date {
                                day: 12,
                                month: 8,
                                year: 2024,
                            },
                            session_id: 1,
                            race_position: 2,
                            car_used: "Kart Type 2".to_string(),
                        },
                        laptimes: vec![Lap {
                            lap_number: 1,
                            time: 28.2,
                        }],
                        ..Default::default()
                    },
                ],
            },
        };

        // When
        karting_time.update_filtering();

        // Then
        assert_eq!(
            expected_races[0],
            karting_time.application_state.filtered_races[0]
        );
    }
}
