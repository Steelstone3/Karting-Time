use crate::models::application::karting_time::KartingTime;
use std::collections::HashSet;

impl KartingTime {
    pub fn get_number_of_races(&self) -> u32 {
        self.application_state.filtered_races.len() as u32
    }

    pub fn get_number_of_wins(&self) -> u32 {
        self.application_state
            .filtered_races
            .iter()
            .filter(|race| race.race_information.race_position == 1)
            .count() as u32
    }

    pub fn get_number_of_podiums(&self) -> u32 {
        self.application_state
            .filtered_races
            .iter()
            .filter(|race| race.race_information.race_position <= 3)
            .count() as u32
    }

    pub fn get_number_of_top_fives(&self) -> u32 {
        self.application_state
            .filtered_races
            .iter()
            .filter(|race| race.race_information.race_position <= 5)
            .count() as u32
    }

    pub fn get_number_of_top_tens(&self) -> u32 {
        self.application_state
            .filtered_races
            .iter()
            .filter(|race| race.race_information.race_position <= 10)
            .count() as u32
    }

    pub fn get_number_of_unique_tracks(&self) -> u32 {
        let unique_tracks: HashSet<String> = self
            .application_state
            .filtered_races
            .iter()
            .map(|race| {
                race.race_information
                    .track_name
                    .trim()
                    .to_lowercase()
                    .clone()
            })
            .collect();

        unique_tracks.len() as u32
    }

    pub fn get_number_of_unique_cars(&self) -> u32 {
        let unique_cars: HashSet<String> = self
            .application_state
            .filtered_races
            .iter()
            .map(|race| race.race_information.car_used.trim().to_lowercase().clone())
            .collect();

        unique_cars.len() as u32
    }
}

#[cfg(test)]
mod driver_statistics_should {
    use crate::models::{
        application::{application_state::ApplicationState, karting_time::KartingTime},
        driver::{race_information::RaceInformation, race_result::Race},
    };

    #[test]
    fn get_number_of_races() {
        // Given
        let expected_number_of_races = 4;

        let karting_time = KartingTime {
            application_state: ApplicationState {
                filtered_races: vec![
                    Race {
                        ..Default::default()
                    },
                    Race {
                        ..Default::default()
                    },
                    Race {
                        ..Default::default()
                    },
                    Race {
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let number_of_races = karting_time.get_number_of_races();

        // Then
        assert_eq!(expected_number_of_races, number_of_races)
    }

    #[test]
    fn get_number_of_wins() {
        // Given
        let expected_number_of_wins = 2;

        let karting_time = KartingTime {
            application_state: ApplicationState {
                filtered_races: vec![
                    Race {
                        race_information: RaceInformation {
                            race_position: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 3,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let number_of_wins = karting_time.get_number_of_wins();

        // Then
        assert_eq!(expected_number_of_wins, number_of_wins)
    }

    #[test]
    fn get_number_of_podiums() {
        // Given
        let expected_number_of_podiums = 3;

        let karting_time = KartingTime {
            application_state: ApplicationState {
                filtered_races: vec![
                    Race {
                        race_information: RaceInformation {
                            race_position: 2,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 3,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let number_of_podiums = karting_time.get_number_of_podiums();

        // Then
        assert_eq!(expected_number_of_podiums, number_of_podiums)
    }

    #[test]
    fn get_number_of_top_fives() {
        // Given
        let expected_number_of_top_fives = 3;

        let karting_time = KartingTime {
            application_state: ApplicationState {
                filtered_races: vec![
                    Race {
                        race_information: RaceInformation {
                            race_position: 1,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 6,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 5,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 3,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let number_of_top_fives = karting_time.get_number_of_top_fives();

        // Then
        assert_eq!(expected_number_of_top_fives, number_of_top_fives)
    }

    #[test]
    fn get_number_of_top_tens() {
        // Given
        let expected_number_of_top_fives = 2;

        let karting_time = KartingTime {
            application_state: ApplicationState {
                filtered_races: vec![
                    Race {
                        race_information: RaceInformation {
                            race_position: 10,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 12,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 11,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            race_position: 3,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let number_of_top_tens = karting_time.get_number_of_top_tens();

        // Then
        assert_eq!(expected_number_of_top_fives, number_of_top_tens)
    }

    #[test]
    fn get_number_of_unique_tracks() {
        // Given
        let track_1 = "Three Sisters".to_string();
        let track_2 = "Llandow".to_string();
        let expected_number_of_unique_tracks = 2;

        let karting_time = KartingTime {
            application_state: ApplicationState {
                filtered_races: vec![
                    Race {
                        race_information: RaceInformation {
                            track_name: track_1.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            track_name: track_1.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            track_name: track_2.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            track_name: track_2.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let number_of_unique_tracks = karting_time.get_number_of_unique_tracks();

        // Then
        assert_eq!(expected_number_of_unique_tracks, number_of_unique_tracks)
    }

    #[test]
    fn get_number_of_unique_cars() {
        // Given
        let car_used_1 = "Go Kart".to_string();
        let car_used_2 = "GT3 Car".to_string();
        let expected_number_of_unique_tracks = 2;

        let karting_time = KartingTime {
            application_state: ApplicationState {
                filtered_races: vec![
                    Race {
                        race_information: RaceInformation {
                            car_used: car_used_1.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            car_used: car_used_1.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            car_used: car_used_2.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    Race {
                        race_information: RaceInformation {
                            car_used: car_used_2.clone(),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        };

        // When
        let number_of_unique_tracks = karting_time.get_number_of_unique_cars();

        // Then
        assert_eq!(expected_number_of_unique_tracks, number_of_unique_tracks)
    }
}
