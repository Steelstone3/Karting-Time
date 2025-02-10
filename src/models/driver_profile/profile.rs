use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::models::driver_results::race_result::Race;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DriverProfile {
    pub name: String,
    pub races: Vec<Race>,
}

impl DriverProfile {
    // TODO Test
    pub fn sort_races(&mut self) {
        self.races.sort_by(|a, b| {
            // First, compare by track_name in ascending order
            let track_name_ordering = a.track_name.cmp(&b.track_name);
            if track_name_ordering != std::cmp::Ordering::Equal {
                return track_name_ordering;
            }

            // If track_names are equal, compare by session_id in ascending order
            let session_id_ordering = a.session_id.cmp(&b.session_id);
            if session_id_ordering != std::cmp::Ordering::Equal {
                return session_id_ordering;
            }

            // If both track_name and session_id are equal, compare by date in *descending* order
            b.date.cmp(&a.date) // Note the reversed order for descending sort
        });
    }

    pub fn get_number_of_races(&self) -> u32 {
        self.races.len() as u32
    }

    pub fn get_number_of_wins(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_position == 1)
            .count() as u32
    }

    pub fn get_number_of_podiums(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_position <= 3)
            .count() as u32
    }

    pub fn get_number_of_top_fives(&self) -> u32 {
        self.races
            .iter()
            .filter(|race| race.race_position <= 5)
            .count() as u32
    }

    pub fn get_number_of_unique_tracks(&self) -> u32 {
        let unique_tracks: HashSet<String> = self
            .races
            .iter()
            .map(|race| race.track_name.trim().to_lowercase().clone())
            .collect();

        unique_tracks.len() as u32
    }
}

#[cfg(test)]
mod driver_profile_should {
    use crate::models::driver_results::race_result::Race;

    use super::DriverProfile;

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
                    race_position: 2,
                    ..Default::default()
                },
                Race {
                    race_position: 5,
                    ..Default::default()
                },
                Race {
                    race_position: 1,
                    ..Default::default()
                },
                Race {
                    race_position: 3,
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
                    race_position: 1,
                    ..Default::default()
                },
                Race {
                    race_position: 6,
                    ..Default::default()
                },
                Race {
                    race_position: 5,
                    ..Default::default()
                },
                Race {
                    race_position: 3,
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
                    track_name: track_1.clone(),
                    ..Default::default()
                },
                Race {
                    track_name: track_1.clone(),
                    ..Default::default()
                },
                Race {
                    track_name: track_2.clone(),
                    ..Default::default()
                },
                Race {
                    track_name: track_2.clone(),
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
