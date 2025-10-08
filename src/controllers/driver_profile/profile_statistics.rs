use crate::models::driver::session_information::race_result::RaceResult;
use std::collections::HashSet;

pub fn get_filtered_number_of_races(filtered_races: &[RaceResult]) -> u32 {
    filtered_races.len() as u32
}

pub fn get_filtered_number_of_wins(filtered_races: &[RaceResult]) -> u32 {
    filtered_races
        .iter()
        .filter(|race| race.race_information.session.race_position == 1)
        .count() as u32
}

pub fn get_filtered_number_of_podiums(filtered_races: &[RaceResult]) -> u32 {
    filtered_races
        .iter()
        .filter(|race| race.race_information.session.race_position <= 3)
        .count() as u32
}

pub fn get_filtered_number_of_top_fives(filtered_races: &[RaceResult]) -> u32 {
    filtered_races
        .iter()
        .filter(|race| race.race_information.session.race_position <= 5)
        .count() as u32
}

pub fn get_filtered_number_of_top_tens(filtered_races: &[RaceResult]) -> u32 {
    filtered_races
        .iter()
        .filter(|race| race.race_information.session.race_position <= 10)
        .count() as u32
}

pub fn get_filtered_number_of_unique_tracks(filtered_races: &[RaceResult]) -> u32 {
    let unique_tracks: HashSet<String> = filtered_races
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

pub fn get_filtered_number_of_unique_cars(filtered_races: &[RaceResult]) -> u32 {
    let unique_cars: HashSet<String> = filtered_races
        .iter()
        .map(|race| race.race_metadata.car_used.trim().to_lowercase().clone())
        .collect();

    unique_cars.len() as u32
}

#[cfg(test)]
mod driver_statistics_should {
    use crate::{
        controllers::driver_profile::profile_statistics::{
            get_filtered_number_of_podiums, get_filtered_number_of_races,
            get_filtered_number_of_top_fives, get_filtered_number_of_top_tens,
            get_filtered_number_of_unique_cars, get_filtered_number_of_unique_tracks,
            get_filtered_number_of_wins,
        },
        models::driver::session_information::{
            race_information::RaceInformation, race_metadata::RaceMetadata,
            race_result::RaceResult, session::Session,
        },
    };

    // TODO AH Test fixtures

    #[test]
    fn get_number_of_races() {
        // Given
        let expected_number_of_races = 4;

        let filtered_races = vec![
            RaceResult::new(Default::default(), Default::default(), Default::default()),
            RaceResult::new(Default::default(), Default::default(), Default::default()),
            RaceResult::new(Default::default(), Default::default(), Default::default()),
            RaceResult::new(Default::default(), Default::default(), Default::default()),
        ];

        // When
        let number_of_races = get_filtered_number_of_races(&filtered_races);

        // Then
        pretty_assertions::assert_eq!(expected_number_of_races, number_of_races)
    }

    #[test]
    fn get_number_of_wins() {
        // Given
        let expected_number_of_wins = 2;

        let filtered_races = vec![
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 1),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 5),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 1),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 3),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
        ];

        // When
        let number_of_wins = get_filtered_number_of_wins(&filtered_races);

        // Then
        pretty_assertions::assert_eq!(expected_number_of_wins, number_of_wins)
    }

    #[test]
    fn get_number_of_podiums() {
        // Given
        let expected_number_of_podiums = 3;

        let filtered_races = vec![
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 2),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 5),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 1),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 3),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
        ];

        // When
        let number_of_podiums = get_filtered_number_of_podiums(&filtered_races);

        // Then
        pretty_assertions::assert_eq!(expected_number_of_podiums, number_of_podiums)
    }

    #[test]
    fn get_number_of_top_fives() {
        // Given
        let expected_number_of_top_fives = 3;

        let filtered_races = vec![
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 1),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 6),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 5),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 3),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
        ];

        // When
        let number_of_top_fives = get_filtered_number_of_top_fives(&filtered_races);

        // Then
        pretty_assertions::assert_eq!(expected_number_of_top_fives, number_of_top_fives)
    }

    #[test]
    fn get_number_of_top_tens() {
        // Given
        let expected_number_of_top_fives = 2;

        let filtered_races = vec![
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 10),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 12),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 11),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), 3),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
        ];

        // When
        let number_of_top_tens = get_filtered_number_of_top_tens(&filtered_races);

        // Then
        pretty_assertions::assert_eq!(expected_number_of_top_fives, number_of_top_tens)
    }

    #[test]
    fn get_number_of_unique_tracks() {
        // Given
        let track_1 = "Three Sisters".to_string();
        let track_2 = "Llandow".to_string();
        let expected_number_of_unique_tracks = 2;

        let filtered_races = vec![
            RaceResult::new(
                RaceInformation::new(
                    &track_1,
                    Default::default(),
                    Session::new(Default::default(), Default::default()),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    &track_1,
                    Default::default(),
                    Session::new(Default::default(), Default::default()),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    &track_2,
                    Default::default(),
                    Session::new(Default::default(), Default::default()),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    &track_2,
                    Default::default(),
                    Session::new(Default::default(), Default::default()),
                ),
                RaceMetadata::default(),
                Default::default(),
            ),
        ];

        // When
        let number_of_unique_tracks = get_filtered_number_of_unique_tracks(&filtered_races);

        // Then
        pretty_assertions::assert_eq!(expected_number_of_unique_tracks, number_of_unique_tracks)
    }

    #[test]
    fn get_number_of_unique_cars() {
        // Given
        let car_used_1 = "Go Kart".to_string();
        let car_used_2 = "GT3 Car".to_string();
        let expected_number_of_unique_tracks = 2;

        let filtered_races = vec![
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), Default::default()),
                ),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    &car_used_1,
                    Default::default(),
                    Default::default(),
                ),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), Default::default()),
                ),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    &car_used_1,
                    Default::default(),
                    Default::default(),
                ),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), Default::default()),
                ),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    &car_used_2,
                    Default::default(),
                    Default::default(),
                ),
                Default::default(),
            ),
            RaceResult::new(
                RaceInformation::new(
                    Default::default(),
                    Default::default(),
                    Session::new(Default::default(), Default::default()),
                ),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    &car_used_2,
                    Default::default(),
                    Default::default(),
                ),
                Default::default(),
            ),
        ];

        // When
        let number_of_unique_tracks = get_filtered_number_of_unique_cars(&filtered_races);

        // Then
        pretty_assertions::assert_eq!(expected_number_of_unique_tracks, number_of_unique_tracks)
    }
}
