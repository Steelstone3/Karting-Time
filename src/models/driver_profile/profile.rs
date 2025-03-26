use crate::{
    data_models::profile_file::DriverProfileFile, models::driver_results::race_result::Race,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DriverProfile {
    pub name: String,
    pub races: Vec<Race>,
}

impl DriverProfile {
    pub fn convert_to_driver_profile_file(&self) -> DriverProfileFile {
        let mut race_files = vec![];

        for race in &self.races {
            race_files.push(race.convert_to_race_file())
        }

        DriverProfileFile {
            name: self.name.to_string(),
            races: race_files,
        }
    }

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

    pub fn get_number_of_races(&self) -> u32 {
        self.races.len() as u32
    }

    pub fn get_number_of_wins(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_information.race_position == 1)
            .count() as u32
    }

    pub fn get_number_of_podiums(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_information.race_position <= 3)
            .count() as u32
    }

    pub fn get_number_of_top_fives(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_information.race_position <= 5)
            .count() as u32
    }

    pub fn get_number_of_unique_tracks(&self) -> u32 {
        let unique_tracks: HashSet<String> = self
            .races
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
}

#[cfg(test)]
mod driver_profile_should {
    use super::DriverProfile;
    use crate::models::driver_results::{race_information::RaceInformation, race_result::Race};

    #[test]
    #[ignore = "reason"]
    fn convert_to_driver_profile_file() {

        //  // TODO Test
        //  pub fn convert_to_driver_profile_file(&self) -> DriverProfileFile {
        //     let mut race_files = vec![];

        //     for race in &self.races {
        //         race_files.push(race.convert_to_race_file())
        //     }

        //     DriverProfileFile {
        //         name: self.name.to_string(),
        //         races: race_files,
        //     }
        // }
    }

    #[test]
    #[ignore = "reason"]
    fn sort_races() {

        // // TODO Test
        // pub fn sort_races(&mut self) {
        //     self.races.sort_by(|a, b| {
        //         b.race_information
        //             .date
        //             .cmp(&a.race_information.date)
        //             .then_with(|| {
        //                 a.race_information
        //                     .track_name
        //                     .cmp(&b.race_information.track_name)
        //             })
        //             .then_with(|| {
        //                 a.race_information
        //                     .session_id
        //                     .cmp(&b.race_information.session_id)
        //             })
        //     });
        // }
    }

    #[test]
    fn get_number_of_races() {
        // Given
        let expected_number_of_races = 4;
        let driver_profile = DriverProfile {
            races: vec![
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
        };

        // When
        let number_of_races = driver_profile.get_number_of_races();

        // Then
        assert_eq!(expected_number_of_races, number_of_races)
    }

    #[test]
    fn get_number_of_podiums() {
        // Given
        let expected_number_of_podiums = 3;
        let driver_profile = DriverProfile {
            races: vec![
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
        };

        // When
        let number_of_podiums = driver_profile.get_number_of_podiums();

        // Then
        assert_eq!(expected_number_of_podiums, number_of_podiums)
    }

    #[test]
    fn get_number_of_top_fives() {
        // Given
        let expected_number_of_top_fives = 3;
        let driver_profile = DriverProfile {
            races: vec![
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
        };

        // When
        let number_of_top_fives = driver_profile.get_number_of_top_fives();

        // Then
        assert_eq!(expected_number_of_top_fives, number_of_top_fives)
    }

    #[test]
    fn get_number_of_unique_tracks() {
        // Given
        let track_1 = "Three Sisters".to_string();
        let track_2 = "Llandow".to_string();
        let expected_number_of_unique_tracks = 2;
        let driver_profile = DriverProfile {
            races: vec![
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
        };

        // When
        let number_of_unique_tracks = driver_profile.get_number_of_unique_tracks();

        // Then
        assert_eq!(expected_number_of_unique_tracks, number_of_unique_tracks)
    }
}
