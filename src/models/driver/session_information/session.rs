use crate::views::application::input_parser::parse_input_u32;

#[derive(Debug, Clone, PartialEq)]
pub struct Session {
    pub session_id: u32,
    pub race_position: u32,
}

impl Default for Session {
    fn default() -> Self {
        Self {
            session_id: 1,
            race_position: 1,
        }
    }
}

impl Session {
    pub fn new(session_id: u32, race_position: u32) -> Self {
        Self {
            session_id,
            race_position,
        }
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.session_id = parse_input_u32(session_id, 1, u32::MAX);
    }

    pub fn set_race_position(&mut self, race_position: String) {
        self.race_position = parse_input_u32(race_position, 1, u32::MAX);
    }
}

#[cfg(test)]
mod session_should {
    use crate::models::driver::session_information::session::Session;

    #[test]
    fn update_session_id() {
        // Given
        let session_id = 5;
        let mut session = Session {
            session_id: 1,
            race_position: 12,
        };

        // When
        session.set_session_id(session_id.to_string());

        // Then
        pretty_assertions::assert_eq!(session_id, session.session_id)
    }

    #[test]
    fn update_race_position() {
        // Given
        let race_position = 5;
        let mut session = Session {
            session_id: 1,
            race_position: 12,
        };

        // When
        session.set_race_position(race_position.to_string());

        // Then
        pretty_assertions::assert_eq!(race_position, session.race_position)
    }
}
