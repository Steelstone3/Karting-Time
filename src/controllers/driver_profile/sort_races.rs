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
                        .session
                        .session_id
                        .cmp(&b.race_information.session.session_id)
                })
        });
    }
}

#[cfg(test)]
mod sort_races_should {
    use crate::models::{
        date::RaceDate,
        driver::{
            driver_profile::DriverProfile,
            session_information::{
                lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
                race_result::RaceResult, session::Session,
            },
        },
    };

    #[test]
    fn sort_races() {
        // Given
        let race_1 = RaceResult::new(
            RaceInformation::new(
                "Three Ponies",
                RaceDate::new(15, 10, 2024),
                Session::new(1, 2),
            ),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                "Kart",
                "Championship",
                Default::default(),
            ),
            vec![Lap::new(1, 50.662), Lap::new(2, 51.877)],
        );

        let race_2 = RaceResult::new(
            RaceInformation::new(
                "Three Ponies",
                RaceDate::new(15, 10, 2024),
                Session::new(2, 1),
            ),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                "Kart",
                "Championship",
                Default::default(),
            ),
            vec![Lap::new(1, 50.723), Lap::new(2, 51.956)],
        );

        let race_3 = RaceResult::new(
            RaceInformation::new(
                "Trafford Stadium",
                RaceDate::new(17, 10, 2024),
                Session::new(1, 1),
            ),
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                "Kart",
                Default::default(),
                Default::default(),
            ),
            vec![Lap::new(1, 30.723), Lap::new(2, 31.956)],
        );

        let mut driver_profile = DriverProfile::new(
            "Karl Chadwick",
            vec![race_1.clone(), race_2.clone(), race_3.clone()],
        );

        // When
        driver_profile.sort_races();

        // Then
        pretty_assertions::assert_eq!(race_3.clone(), driver_profile.races[0]);
        pretty_assertions::assert_eq!(race_1.clone(), driver_profile.races[1]);
        pretty_assertions::assert_eq!(race_2.clone(), driver_profile.races[2]);
    }
}
