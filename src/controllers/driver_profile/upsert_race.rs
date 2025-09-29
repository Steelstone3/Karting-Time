use crate::models::driver::driver_profile::DriverProfile;

impl DriverProfile {
    pub fn upsert_race(&mut self) {
        match self.new_race.is_unique_identifer(&self.races) {
            true => {
                self.session_type_default();
                self.track_condition_default();
                self.car_used_default();

                self.races.push(self.new_race.clone());
            }
            false => {
                self.races = self.new_race.replace_existing_race(&self.races);
            }
        }
    }

    fn session_type_default(&mut self) {
        if self.new_race.race_metadata.session_type.is_empty() {
            self.new_race.race_metadata.session_type = "N/A".to_string();
        }
    }

    fn track_condition_default(&mut self) {
        if self.new_race.race_metadata.track_conditions.is_empty() {
            self.new_race.race_metadata.track_conditions = "N/A".to_string();
        }
    }

    fn car_used_default(&mut self) {
        if self.new_race.race_metadata.car_used.is_empty() {
            self.new_race.race_metadata.car_used = "N/A".to_string();
        }
    }
}

#[cfg(test)]
mod upsert_race_should {
    use crate::models::{
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

        let new_race = RaceResult::new(
            RaceInformation::new(
                "Three Sisters",
                RaceDate::new(12, 11, 2023),
                Session::new(2, 12),
            ),
            RaceMetadata::default(),
            Default::default(),
        );

        let mut driver_profile =
            DriverProfile::new_with_new_race(Default::default(), new_race, Default::default());

        // When
        driver_profile.upsert_race();

        // Then
        pretty_assertions::assert_eq!(
            expected_race.race_information,
            driver_profile.races[0].race_information
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

        let mut driver_profile =
            DriverProfile::new_with_new_race(Default::default(), race.clone(), Default::default());

        // When
        driver_profile.upsert_race();

        // Then
        pretty_assertions::assert_eq!(race, driver_profile.races[0])
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

        let mut driver_profile =
            DriverProfile::new(Default::default(), vec![expected_race.clone()]);

        // When
        driver_profile.upsert_race();

        // Then
        pretty_assertions::assert_eq!(1, driver_profile.races.len());
        pretty_assertions::assert_eq!(expected_race, driver_profile.races[0])
    }
}
