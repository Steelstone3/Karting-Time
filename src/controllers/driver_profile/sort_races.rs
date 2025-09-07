use crate::models::driver::driver_profile::DriverProfile;

impl DriverProfile {
    pub fn sort_races(&mut self) {
        self.races.sort_by(|a, b| {
            b.race_information
                .date
                .cmp(&a.race_information.date)
                .then_with(|| {
                    a.race_information
                        .track_name
                        .cmp(&b.race_information.track_name)
                })
                .then_with(|| {
                    a.race_information
                        .session_id
                        .cmp(&b.race_information.session_id)
                })
        });
    }
}

#[cfg(test)]
mod sort_races_should {
    use crate::models::{
        date::Date,
        driver::{
            driver_profile::DriverProfile, lap::Lap, race_information::RaceInformation,
            race_result::Race,
        },
    };

    #[test]
    fn sort_races() {
        // Given
        let race_1 = Race {
            race_information: RaceInformation {
                track_name: "Three Ponies".to_string(),
                date: Date {
                    day: 15,
                    month: 10,
                    year: 2024,
                },
                session_id: 1,
                race_position: 2,
                car_used: "Kart".to_string(),
                notes: Default::default()
            },
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 50.662,
                },
                Lap {
                    lap_number: 2,
                    time: 51.877,
                },
            ],
            ..Default::default()
        };

        let race_2 = Race {
            race_information: RaceInformation {
                track_name: "Three Ponies".to_string(),
                date: Date {
                    day: 15,
                    month: 10,
                    year: 2024,
                },
                session_id: 2,
                race_position: 1,
                car_used: "Kart".to_string(),
                notes: Default::default()
            },
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 50.723,
                },
                Lap {
                    lap_number: 2,
                    time: 51.956,
                },
            ],
            ..Default::default()
        };

        let race_3 = Race {
            race_information: RaceInformation {
                track_name: "Trafford Stadium".to_string(),
                date: Date {
                    day: 17,
                    month: 10,
                    year: 2024,
                },
                session_id: 1,
                race_position: 1,
                car_used: "Kart".to_string(),
                notes: Default::default()
            },
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 30.723,
                },
                Lap {
                    lap_number: 2,
                    time: 31.956,
                },
            ],
            ..Default::default()
        };

        let mut driver_profile = DriverProfile {
            name: "Karl Chadwick".to_string(),
            races: vec![race_1.clone(), race_2.clone(), race_3.clone()],
        };

        // When
        driver_profile.sort_races();

        // Then
        assert_eq!(race_3.clone(), driver_profile.races[0]);
        assert_eq!(race_1.clone(), driver_profile.races[1]);
        assert_eq!(race_2.clone(), driver_profile.races[2]);
    }
}
