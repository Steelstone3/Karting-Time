use crate::models::driver::{race_information::RaceInformation, race_result::Race};

impl Race {
    pub fn is_unique_identifer(&self, races: &Vec<Race>) -> bool {
        for race in races {
            if RaceInformation::get_unique_race_information_identifier(&self.race_information)
                == RaceInformation::get_unique_race_information_identifier(&race.race_information)
                || self.race_information.track_name.is_empty()
            {
                return false;
            }
        }

        true
    }

    pub fn replace_existing_race(&self, races: &[Race]) -> Vec<Race> {
        let mut updated_races = races.to_owned();

        for i in 0..updated_races.len() {
            if RaceInformation::get_unique_race_information_identifier(&self.race_information)
                == RaceInformation::get_unique_race_information_identifier(
                    &updated_races[i].race_information,
                )
            {
                updated_races[i] = self.clone();
                return updated_races;
            }
        }

        updated_races
    }
}

#[cfg(test)]
mod update_existing_race {
    use rstest::rstest;

    use crate::models::{
        date::Date,
        driver::{lap::Lap, race_information::RaceInformation, race_result::Race},
    };

    #[rstest]
    #[case(Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() },Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() },Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() }, false)]
    #[case(Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() },Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:16,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() },Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:17,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() }, true)]
    #[case(Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() },Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:2,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() },Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:3,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() }, true)]
    #[case(Race{ race_information: RaceInformation {track_name:"Three Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() },Race{ race_information: RaceInformation {track_name:"Four Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() },Race{ race_information: RaceInformation {track_name:"Five Sisters".to_string(),date:Date{day:15,month:11,year:2025},session_id:1,race_position:1,car_used:"Kart".to_string(),notes:Default::default(), session_type: "N/A".to_string(), track_conditions: "N/A".to_string() }, ..Default::default() }, true)]
    fn is_unique_identifer(
        #[case] race_1: Race,
        #[case] race_2: Race,
        #[case] race_3: Race,
        #[case] expected_is_unique: bool,
    ) {
        // Given
        let races = vec![race_2, race_3];

        // When
        let is_unique = race_1.is_unique_identifer(&races);

        // Then
        assert_eq!(expected_is_unique, is_unique)
    }

    #[test]
    fn replace_existing_race() {
        // Given
        let updated_race = Race {
            race_information: RaceInformation {
                track_name: "Trafford Park".to_string(),
                date: Date {
                    day: 15,
                    month: 11,
                    year: 2025,
                },
                session_id: 1,
                session_type: "N/A".to_string(),
                track_conditions: "N/A".to_string(),

                race_position: 5,
                car_used: "Kart".to_string(),
                notes: Default::default(),
            },
            laptimes: vec![Lap {
                lap_number: 1,
                time: 10.5,
            }],
            ..Default::default()
        };

        let existing_races = vec![
            Race {
                race_information: RaceInformation {
                    track_name: "Trafford Park".to_string(),
                    date: Date {
                        day: 15,
                        month: 11,
                        year: 2025,
                    },
                    session_id: 1,
                    session_type: "N/A".to_string(),
                    track_conditions: "N/A".to_string(),
                    race_position: 3,
                    car_used: "Kart".to_string(),
                    notes: Default::default(),
                },
                laptimes: vec![Lap {
                    lap_number: 1,
                    time: 9.5,
                }],
                ..Default::default()
            },
            Race {
                race_information: RaceInformation {
                    track_name: "Trafford Park".to_string(),
                    date: Date {
                        day: 15,
                        month: 11,
                        year: 2025,
                    },
                    session_id: 2,
                    session_type: "N/A".to_string(),
                    track_conditions: "N/A".to_string(),
                    race_position: 5,
                    car_used: "Kart".to_string(),
                    notes: Default::default(),
                },
                laptimes: vec![Lap {
                    lap_number: 1,
                    time: 10.5,
                }],
                ..Default::default()
            },
        ];

        let expected_updated_existing_races = vec![
            Race {
                race_information: RaceInformation {
                    track_name: "Trafford Park".to_string(),
                    date: Date {
                        day: 15,
                        month: 11,
                        year: 2025,
                    },
                    session_id: 1,
                    session_type: "N/A".to_string(),
                    track_conditions: "N/A".to_string(),
                    race_position: 5,
                    car_used: "Kart".to_string(),
                    notes: Default::default(),
                },
                laptimes: vec![Lap {
                    lap_number: 1,
                    time: 10.5,
                }],
                ..Default::default()
            },
            Race {
                race_information: RaceInformation {
                    track_name: "Trafford Park".to_string(),
                    date: Date {
                        day: 15,
                        month: 11,
                        year: 2025,
                    },
                    session_id: 2,
                    session_type: "N/A".to_string(),
                    track_conditions: "N/A".to_string(),
                    race_position: 5,
                    car_used: "Kart".to_string(),
                    notes: Default::default(),
                },
                laptimes: vec![Lap {
                    lap_number: 1,
                    time: 10.5,
                }],
                ..Default::default()
            },
        ];

        // When
        let updated_existing_races = updated_race.replace_existing_race(&existing_races);

        // Then
        assert_eq!(expected_updated_existing_races, updated_existing_races)
    }
}
