use crate::models::driver::session_information::race_result::RaceResult;

impl RaceResult {
    pub fn is_unique_identifer(&self, races: &Vec<RaceResult>) -> bool {
        for race in races {
            if self.race_information.unique_race_identifier
                == race.race_information.unique_race_identifier
                || self.race_information.track_name.is_empty()
            {
                return false;
            }
        }

        true
    }

    pub fn replace_existing_race(&self, races: &[RaceResult]) -> Vec<RaceResult> {
        let mut updated_races = races.to_owned();

        for i in 0..updated_races.len() {
            if self.race_information.unique_race_identifier
                == updated_races[i].race_information.unique_race_identifier
            {
                updated_races[i] = RaceResult::new_from_self(self.clone());
                return updated_races;
            }
        }

        updated_races
    }
}

#[cfg(test)]
mod update_existing_race {
    use crate::models::{
        date::RaceDate,
        driver::session_information::{
            lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
            race_result::RaceResult, session::Session,
        },
    };
    use rstest::rstest;

    #[rstest]
    #[case("Three Sisters", "Three Sisters", "Three Sisters", RaceDate{day:15,month:11,year:2025}, RaceDate{day:15,month:11,year:2025}, RaceDate{day:15,month:11,year:2025}, 1, 1, 1, false)]
    #[case("Three Sisters", "Four Sisters", "Five Sisters", RaceDate{day:15,month:11,year:2025}, RaceDate{day:16,month:11,year:2025}, RaceDate{day:17,month:11,year:2025}, 1, 2, 3, true)]
    #[case("Three Sisters", "Four Sisters", "Five Sisters", RaceDate{day:15,month:11,year:2025}, RaceDate{day:15,month:11,year:2025}, RaceDate{day:15,month:11,year:2025}, 1, 1, 1, true)]
    #[case("Three Sisters", "Three Sisters", "Three Sisters", RaceDate{day:15,month:11,year:2025}, RaceDate{day:16,month:11,year:2025}, RaceDate{day:17,month:11,year:2025}, 1, 1, 1, true)]
    #[case("Three Sisters", "Three Sisters", "Three Sisters", RaceDate{day:15,month:11,year:2025}, RaceDate{day:15,month:11,year:2025}, RaceDate{day:15,month:11,year:2025}, 1, 2, 3, true)]
    fn is_unique_identifer(
        #[case] track_1: String,
        #[case] track_2: String,
        #[case] track_3: String,
        #[case] race_date_1: RaceDate,
        #[case] race_date_2: RaceDate,
        #[case] race_date_3: RaceDate,
        #[case] session_id_1: u32,
        #[case] session_id_2: u32,
        #[case] session_id_3: u32,
        #[case] expected_is_unique: bool,
    ) {
        // Given

        let race_1 = RaceResult::new(
            RaceInformation::new(&track_1, race_date_1, Session::new(session_id_1, 1)),
            Default::default(),
            Default::default(),
        );
        let race_2 = RaceResult::new(
            RaceInformation::new(&track_2, race_date_2, Session::new(session_id_2, 1)),
            Default::default(),
            Default::default(),
        );
        let race_3 = RaceResult::new(
            RaceInformation::new(&track_3, race_date_3, Session::new(session_id_3, 1)),
            Default::default(),
            Default::default(),
        );
        let races = vec![race_2, race_3];

        // When
        let is_unique = race_1.is_unique_identifer(&races);

        // Then
        pretty_assertions::assert_eq!(expected_is_unique, is_unique)
    }

    #[test]
    fn replace_existing_race() {
        // Given
        let updated_race = RaceResult::new(
            RaceInformation::new(
                "Trafford Park",
                RaceDate::new(15, 11, 2025),
                Session::new(1, 5),
            ),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                "Kart",
                Default::default(),
                Default::default(),
            ),
            vec![Lap {
                lap_number: 1,
                time: 10.5,
            }],
        );

        let existing_races = vec![
            RaceResult::new(
                RaceInformation::new(
                    "Trafford Park",
                    RaceDate::new(15, 11, 2025),
                    Session::new(1, 3),
                ),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    "Kart",
                    Default::default(),
                    Default::default(),
                ),
                vec![Lap::new(1, 9.5)],
            ),
            RaceResult::new(
                RaceInformation::new(
                    "Trafford Park",
                    RaceDate::new(15, 11, 2025),
                    Session::new(2, 5),
                ),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    "Kart",
                    Default::default(),
                    Default::default(),
                ),
                vec![Lap::new(1, 10.5)],
            ),
        ];

        let expected_updated_existing_races = vec![
            RaceResult::new(
                RaceInformation::new(
                    "Trafford Park",
                    RaceDate::new(15, 11, 2025),
                    Session::new(1, 5),
                ),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    "Kart",
                    Default::default(),
                    Default::default(),
                ),
                vec![Lap::new(1, 10.5)],
            ),
            RaceResult::new(
                RaceInformation::new(
                    "Trafford Park",
                    RaceDate::new(15, 11, 2025),
                    Session::new(2, 5),
                ),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    "Kart",
                    Default::default(),
                    Default::default(),
                ),
                vec![Lap::new(1, 10.5)],
            ),
        ];

        // When
        let updated_existing_races = updated_race.replace_existing_race(&existing_races);

        // Then
        pretty_assertions::assert_eq!(expected_updated_existing_races, updated_existing_races)
    }
}
