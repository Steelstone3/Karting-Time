use crate::models::application::karting_time::KartingTime;

impl KartingTime {
    pub fn upsert_race(&mut self) {
        match self
            .application_state
            .new_race
            .is_unique_identifer(&self.driver_profile.races)
        {
            true => {
                self.session_type_default();
                self.track_condition_default();
                self.car_used_default();

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

    fn session_type_default(&mut self) {
        if self
            .application_state
            .new_race
            .race_information
            .session
            .session_type
            .is_empty()
        {
            self.application_state
                .new_race
                .race_information
                .session
                .session_type = "N/A".to_string();
        }
    }

    fn track_condition_default(&mut self) {
        if self
            .application_state
            .new_race
            .race_information
            .session
            .track_condition
            .is_empty()
        {
            self.application_state
                .new_race
                .race_information
                .session
                .track_condition = "N/A".to_string();
        }
    }

    fn car_used_default(&mut self) {
        if self
            .application_state
            .new_race
            .race_information
            .car_used
            .is_empty()
        {
            self.application_state.new_race.race_information.car_used = "N/A".to_string();
        }
    }
}

#[cfg(test)]
mod upsert_race_should {
    use crate::models::{
        application::{application_state::ApplicationState, karting_time::KartingTime},
        date::Date,
        driver::{
            driver_profile::DriverProfile, race_information::RaceInformation, race_result::Race,
            session::Session,
        },
    };

    #[test]
    fn upsert_race_no_session_type_or_track_condition_or_car_used() {
        // Given
        let expected_race = Race {
            race_information: RaceInformation {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 12,
                    month: 11,
                    year: 2023,
                },
                session: Session {
                    session_id: 2,
                    session_type: "N/A".to_string(),
                    track_condition: "N/A".to_string(),
                    race_position: 12,
                },
                car_used: "N/A".to_string(),
                championship: "".to_string(),
                notes: "".to_string(),
            },
            ..Default::default()
        };
        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                new_race: Race {
                    race_information: RaceInformation {
                        track_name: "Three Sisters".to_string(),
                        date: Date {
                            day: 12,
                            month: 11,
                            year: 2023,
                        },
                        session: Session {
                            session_id: 2,
                            session_type: "".to_string(),
                            track_condition: "".to_string(),
                            race_position: 12,
                        },
                        car_used: "".to_string(),
                        championship: "".to_string(),
                        notes: "".to_string(),
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            driver_profile: DriverProfile {
                races: Default::default(),
                ..Default::default()
            },
        };

        // When
        karting_time.upsert_race();

        // Then
        assert_eq!(
            expected_race.race_information,
            karting_time.driver_profile.races[0].race_information
        )
    }

    #[test]
    fn upsert_race() {
        // Given
        let expected_race = Race {
            race_information: RaceInformation {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 12,
                    month: 11,
                    year: 2023,
                },
                session: Session {
                    session_id: 2,
                    session_type: "Race".to_string(),
                    track_condition: "Dry".to_string(),
                    race_position: 12,
                },
                car_used: "Kart Type 1".to_string(),
                championship: "Championship".to_string(),
                notes: "Some notes".to_string(),
            },
            ..Default::default()
        };
        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                new_race: Race {
                    race_information: RaceInformation {
                        track_name: "Three Sisters".to_string(),
                        date: Date {
                            day: 12,
                            month: 11,
                            year: 2023,
                        },
                        session: Session {
                            session_id: 2,
                            session_type: "Race".to_string(),
                            track_condition: "Dry".to_string(),
                            race_position: 12,
                        },
                        car_used: "Kart Type 1".to_string(),
                        championship: "Championship".to_string(),
                        notes: "Some notes".to_string(),
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            driver_profile: DriverProfile {
                races: Default::default(),
                ..Default::default()
            },
        };

        // When
        karting_time.upsert_race();

        // Then
        assert_eq!(
            expected_race.race_information,
            karting_time.driver_profile.races[0].race_information
        )
    }

    #[test]
    fn upsert_race_match_and_update_existing() {
        // Given
        let expected_race = Race {
            race_information: RaceInformation {
                track_name: "Three Sisters".to_string(),
                date: Date {
                    day: 12,
                    month: 11,
                    year: 2023,
                },
                session: Session {
                    session_id: 2,
                    session_type: "Practise".to_string(),
                    track_condition: "Wet".to_string(),
                    race_position: 8,
                },
                car_used: "Kart Type 2".to_string(),
                championship: "Championship".to_string(),
                notes: "Some different notes".to_string(),
            },
            ..Default::default()
        };
        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                new_race: Race {
                    race_information: RaceInformation {
                        track_name: "Three Sisters".to_string(),
                        date: Date {
                            day: 12,
                            month: 11,
                            year: 2023,
                        },
                        session: Session {
                            session_id: 2,
                            session_type: "Practise".to_string(),
                            track_condition: "Wet".to_string(),
                            race_position: 8,
                        },
                        car_used: "Kart Type 2".to_string(),
                        championship: "Championship".to_string(),
                        notes: "Some different notes".to_string(),
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            driver_profile: DriverProfile {
                races: vec![Race {
                    race_information: RaceInformation {
                        track_name: "Three Sisters".to_string(),
                        date: Date {
                            day: 12,
                            month: 11,
                            year: 2023,
                        },
                        session: Session {
                            session_id: 2,
                            session_type: "Race".to_string(),
                            track_condition: "Dry".to_string(),
                            race_position: 12,
                        },
                        car_used: "Kart Type 1".to_string(),
                        championship: "Championship".to_string(),
                        notes: "Some notes".to_string(),
                    },
                    ..Default::default()
                }],
                ..Default::default()
            },
        };

        // When
        karting_time.upsert_race();

        // Then
        assert_eq!(1, karting_time.driver_profile.races.len());
        assert_eq!(expected_race, karting_time.driver_profile.races[0])
    }
}
