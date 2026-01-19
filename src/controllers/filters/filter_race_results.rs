use crate::models::driver::driver_profile::DriverProfile;

impl DriverProfile {
    pub fn update_filtering(&mut self) {
        match self.filter.is_filter_visible {
            true => self.apply_filters(),
            false => self.filter.filtered_races = self.races.clone(),
        }

        self.update_driver_profile()
    }

    fn apply_filters(&mut self) {
        self.filter.filtered_races = self.races.clone();
        if !self.is_all_filters_empty() || self.is_any_filters_used() {
            self.all_filters();
        } else if self.is_all_filters_empty() {
            self.filter.filtered_races = self.races.clone();
        }
    }

    fn track_name_filter(&mut self) {
        let query = self.filter.track_query.to_lowercase();
        self.filter.filtered_races = self
            .filter
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
        let query = self.filter.date_query.to_lowercase();
        self.filter.filtered_races = self
            .filter
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
        let query = self.filter.car_used_query.to_lowercase();
        self.filter.filtered_races = self
            .filter
            .filtered_races
            .iter()
            .filter(|race| {
                // car used matches
                race.race_metadata
                    .car_used
                    .to_string()
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect()
    }

    fn championship_filter(&mut self) {
        let query = self.filter.championship_query.to_lowercase();
        self.filter.filtered_races = self
            .filter
            .filtered_races
            .iter()
            .filter(|race| {
                // championship matches
                race.race_metadata
                    .championship
                    .to_string()
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect()
    }

    fn session_type_filter(&mut self) {
        let query = self.filter.session_type_query.to_lowercase();
        self.filter.filtered_races = self
            .filter
            .filtered_races
            .iter()
            .filter(|race| {
                // session type matches
                race.race_metadata
                    .session_type
                    .to_string()
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect()
    }

    fn is_all_filters_empty(&self) -> bool {
        self.filter.track_query.is_empty()
            && self.filter.date_query.is_empty()
            && self.filter.car_used_query.is_empty()
            && self.filter.championship_query.is_empty()
            && self.filter.session_type_query.is_empty()
    }

    fn is_any_filters_used(&self) -> bool {
        !self.filter.track_query.is_empty()
            || !self.filter.date_query.is_empty()
            || !self.filter.car_used_query.is_empty()
            || !self.filter.championship_query.is_empty()
            || !self.filter.session_type_query.is_empty()
    }

    fn all_filters(&mut self) {
        self.track_name_filter();
        self.date_filter();
        self.car_used_filter();
        self.championship_filter();
        self.session_type_filter();
    }
}

#[cfg(test)]
mod filter_race_results_should {
    use crate::models::date::RaceDate;
    use crate::models::driver::driver_profile::DriverProfile;
    use crate::models::driver::session_information::{
        race_information::RaceInformation, race_metadata::RaceMetadata, race_result::RaceResult,
        session::Session,
    };
    use rstest::rstest;

    #[rstest]
    #[case(false, "", "", "", "", "", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 2)]
    #[case(true, "Silverstone", "", "", "", "", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "", "2025-12-21", "", "", "", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "", "", "MX5", "", "", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "", "", "", "MX5 Cup", "", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "", "", "", "", "Race", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "Silverstone", "", "", "", "Race", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "", "", "MX5", "", "Race", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "", "", "MX5", "MX5 Cup", "", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "", "2025-12-21", "MX5", "MX5 Cup", "", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "", "2025-12-21", "MX5", "MX5 Cup", "Race", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    #[case(true, "Silverstone", "2025-12-21", "MX5", "MX5 Cup", "Race", RaceDate{ day: 21, month: 12, year: 2025 }, "Silverstone", "MX5", "MX5 Cup", "Race", 1)]
    fn update_filtering(
        #[case] is_filter_visible: bool,
        #[case] track_query: String,
        #[case] date_query: String,
        #[case] car_used_query: String,
        #[case] championship_query: String,
        #[case] session_type_query: String,
        #[case] date: RaceDate,
        #[case] track_name: String,
        #[case] car_used: String,
        #[case] championship: String,
        #[case] session_type: String,
        #[case] count: usize,
    ) {
        // Given

        use crate::models::filters::filter::Filter;
        let races = vec![
            RaceResult::new(
                RaceInformation::new(&track_name, date, Session::new(1, 2)),
                RaceMetadata::new(
                    &session_type,
                    Default::default(),
                    &car_used,
                    &championship,
                    Default::default(),
                ),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    "King Derek's Track",
                    RaceDate::new(12, 12, 1066),
                    Session::new(2, 15),
                ),
                RaceMetadata::new(
                    "Horse Qually",
                    Default::default(),
                    "Neigh So Good",
                    "Clop Clop Championship",
                    Default::default(),
                ),
                Default::default(),
            ),
        ];

        let mut driver_profile = DriverProfile::new(Default::default(), races.clone());

        driver_profile.filter = Filter::new(
            is_filter_visible,
            &track_query,
            &date_query,
            &car_used_query,
            &championship_query,
            &session_type_query,
            races.clone(),
        );

        // When
        driver_profile.update_filtering();

        // Then
        pretty_assertions::assert_eq!(count, driver_profile.filter.filtered_races.len());
        pretty_assertions::assert_eq!(races[0], driver_profile.filter.filtered_races[0]);
    }
}
