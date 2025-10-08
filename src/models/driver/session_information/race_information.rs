use crate::models::{date::RaceDate, driver::session_information::session::Session};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct RaceInformation {
    pub unique_race_identifier: String,
    pub track_name: String,
    pub date: RaceDate,
    pub session: Session,
}

impl RaceInformation {
    pub fn new(track_name: &str, date: RaceDate, session: Session) -> Self {
        let mut race_information = Self {
            track_name: track_name.to_string(),
            date,
            session,
            unique_race_identifier: Default::default(),
        };

        let mut unique_race_identifier = "".to_string();

        if !track_name.is_empty() {
            unique_race_identifier = format!(
                "Date_{}_Track_{}_Session_{}",
                race_information.date,
                race_information.track_name,
                race_information.session.session_id
            );
        }

        race_information.unique_race_identifier = unique_race_identifier;

        race_information
    }

    pub fn update_unique_identifier(&mut self) {
        self.unique_race_identifier = format!(
            "Date_{}_Track_{}_Session_{}",
            self.date, self.track_name, self.session.session_id
        );
    }
}

#[cfg(test)]
mod race_information_should {
    use crate::models::{date::RaceDate, driver::session_information::session::Session};

    use super::RaceInformation;

    #[test]
    fn create_race_information() {
        // Given
        let expected_race_information = RaceInformation {
            unique_race_identifier: Default::default(),
            track_name: Default::default(),
            date: Default::default(),
            session: Default::default(),
        };

        // Then
        pretty_assertions::assert_eq!(expected_race_information, RaceInformation::default())
    }

    #[test]
    fn get_unique_race_identifier() {
        // Given
        let expected_race_identifier = "Date_2024-10-15_Track_Three Sisters_Session_1".to_string();
        let race_information = RaceInformation::new(
            "Three Sisters",
            RaceDate {
                day: 15,
                month: 10,
                year: 2024,
            },
            Session {
                session_id: 1,
                race_position: 12,
            },
        );

        // Then
        pretty_assertions::assert_eq!(
            expected_race_identifier,
            race_information.unique_race_identifier
        )
    }
}
