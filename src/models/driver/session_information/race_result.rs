use super::{lap::Lap, race_information::RaceInformation};
use crate::{
    data_models::race_file::RaceFile,
    models::driver::session_information::{
        race_metadata::RaceMetadata, race_statistics::RaceStatistics, session::Session,
    },
};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RaceResult {
    pub race_information: RaceInformation,
    pub race_metadata: RaceMetadata,
    pub race_statistics: RaceStatistics,
    pub laptimes: Vec<Lap>,
    pub is_deleting: bool,
}

impl RaceResult {
    pub fn new(
        race_information: RaceInformation,
        race_metadata: RaceMetadata,
        laptimes: Vec<Lap>,
    ) -> Self {
        let mut race_result = Self {
            race_information,
            race_metadata,
            race_statistics: Default::default(),
            laptimes,
            is_deleting: false,
        };

        race_result.with_race_statistics(RaceStatistics::new(&race_result));

        race_result
    }

    pub fn convert_to_race_file(&self) -> RaceFile {
        RaceFile::new(
            &self.race_information.track_name,
            self.convert_laps_to_laptimes(),
            RaceMetadata::new(
                &self.race_metadata.session_type,
                &self.race_metadata.track_conditions,
                &self.race_metadata.car_used,
                &self.race_metadata.championship,
                &self.race_metadata.notes,
            ),
            Session::new(
                self.race_information.session.session_id,
                self.race_information.session.race_position,
            ),
            self.race_information.date.clone(),
        )
    }

    fn with_race_statistics(&mut self, race_statistics: RaceStatistics) {
        self.race_statistics = race_statistics;
    }

    fn convert_laps_to_laptimes(&self) -> Vec<String> {
        let mut laps: Vec<String> = vec![];

        for laptime in &self.laptimes {
            let time = laptime.time.to_string();

            laps.push(time)
        }

        laps
    }
}

#[cfg(test)]
mod race_result_should {
    use super::*;
    use crate::models::date::RaceDate;

    #[test]
    fn convert_to_race_file() {
        // Given
        let expected_race_file = RaceFile::new(
            "Three Sisters",
            vec!["54.2".to_string(), "55.6".to_string()],
            RaceMetadata::new(
                Default::default(),
                Default::default(),
                "Kart",
                "Championship",
                "Notes",
            ),
            Session::new(2, 7),
            RaceDate::new(15, 11, 2025),
        );

        let race_information = RaceInformation::new(
            "Three Sisters",
            RaceDate::new(15, 11, 2025),
            Session::new(2, 7),
        );
        let race_metadata = RaceMetadata::new(
            Default::default(),
            Default::default(),
            "Kart",
            "Championship",
            "Notes",
        );
        let laptimes = vec![Lap::new(1, 54.2), Lap::new(2, 55.6)];
        let mut race = RaceResult::new(race_information, race_metadata, laptimes.clone());
        let race_statistics = RaceStatistics::new(&race);
        race.with_race_statistics(race_statistics);

        // When
        let race_file = race.convert_to_race_file();

        // Then
        pretty_assertions::assert_eq!(expected_race_file, race_file);
    }

    #[test]
    fn convert_laps_to_laptimes() {
        // Given
        let expected_laptime_string = vec!["10.2".to_string(), "11.8".to_string()];
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![Lap::new(1, 10.2), Lap::new(2, 11.8)],
        );

        // When
        let laptime_string = race.convert_laps_to_laptimes();

        // Then
        pretty_assertions::assert_eq!(expected_laptime_string, laptime_string);
    }
}
