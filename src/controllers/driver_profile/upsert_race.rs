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
            .race_metadata
            .session_type
            .is_empty()
        {
            self.application_state.new_race.race_metadata.session_type = "N/A".to_string();
        }
    }

    fn track_condition_default(&mut self) {
        if self
            .application_state
            .new_race
            .race_metadata
            .track_conditions
            .is_empty()
        {
            self.application_state
                .new_race
                .race_metadata
                .track_conditions = "N/A".to_string();
        }
    }

    fn car_used_default(&mut self) {
        if self
            .application_state
            .new_race
            .race_metadata
            .car_used
            .is_empty()
        {
            self.application_state.new_race.race_metadata.car_used = "N/A".to_string();
        }
    }
}

#[cfg(test)]
mod upsert_race_should {
    use crate::models::{
        application::{application_state::ApplicationState, karting_time::KartingTime},
        date::RaceDate,
        driver::{
            driver_profile::DriverProfile,
            session_information::{
                race_information::RaceInformation, race_metadata::RaceMetadata,
                race_result::RaceResult, session::Session,
            },
        },
    };

    #[test]
    fn upsert_race_no_session_type_or_track_condition_or_car_used() {
        // Given
        let race_information = RaceInformation::new(
            "Three Sisters",
            RaceDate::new(12, 11, 2023),
            Session::new(2, 12),
        );
        let race_metadata = RaceMetadata::default();
        let expected_race = RaceResult::new(race_information, race_metadata, Default::default());

        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                new_race: RaceResult::new(
                    RaceInformation::new(
                        "Three Sisters",
                        RaceDate::new(12, 11, 2023),
                        Session::new(2, 12),
                    ),
                    RaceMetadata::default(),
                    Default::default(),
                ),
                ..Default::default()
            },
            driver_profile: DriverProfile::new(Default::default(), Default::default()),
        };

        // When
        karting_time.upsert_race();

        // Then
        pretty_assertions::assert_eq!(
            expected_race.race_information,
            karting_time.driver_profile.races[0].race_information
        )
    }

    #[test]
    fn upsert_race() {
        // Given
        let race_information = RaceInformation::new(
            "Three Sisters",
            RaceDate::new(12, 11, 2023),
            Session::new(2, 12),
        );
        let race_metadata =
            RaceMetadata::new("Race", "Dry", "Kart Type 1", "Championship", "Some notes");
        let race = RaceResult::new(race_information, race_metadata, Default::default());

        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                new_race: race.clone(),
                ..Default::default()
            },
            driver_profile: DriverProfile::new(Default::default(), Default::default()),
        };

        // When
        karting_time.upsert_race();

        // Then
        pretty_assertions::assert_eq!(race, karting_time.driver_profile.races[0])
    }

    #[test]
    fn upsert_race_match_and_update_existing() {
        // Given
        let race_information = RaceInformation::new(
            "Three Sisters",
            RaceDate::new(12, 11, 2023),
            Session::new(2, 8),
        );
        let race_metadata = RaceMetadata::new(
            "Practise",
            "Wet",
            "Kart Type 2",
            "Championship",
            "Some different notes",
        );
        let expected_race = RaceResult::new(race_information, race_metadata, Default::default());

        let mut karting_time = KartingTime {
            application_state: ApplicationState {
                new_race: expected_race.clone(),
                ..Default::default()
            },
            driver_profile: DriverProfile::new(Default::default(), vec![expected_race.clone()]),
        };

        // When
        karting_time.upsert_race();

        // Then
        pretty_assertions::assert_eq!(1, karting_time.driver_profile.races.len());
        pretty_assertions::assert_eq!(expected_race, karting_time.driver_profile.races[0])
    }
}
