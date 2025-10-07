#[derive(Debug, Clone, PartialEq)]
pub struct RaceMetadata {
    pub session_type: String,
    pub track_conditions: String,
    pub car_used: String,
    pub championship: String,
    pub notes: String,
}

impl RaceMetadata {
    pub fn new(
        mut session_type: &str,
        mut track_conditions: &str,
        car_used: &str,
        championship: &str,
        notes: &str,
    ) -> Self {
        if session_type.is_empty() {
            session_type = "N/A"
        }

        if track_conditions.is_empty() {
            track_conditions = "N/A"
        }

        Self {
            session_type: session_type.to_string(),
            track_conditions: track_conditions.to_string(),
            car_used: car_used.to_string(),
            championship: championship.to_string(),
            notes: notes.to_string(),
        }
    }
}

impl Default for RaceMetadata {
    fn default() -> Self {
        Self {
            session_type: "N/A".to_string(),
            track_conditions: "N/A".to_string(),
            car_used: "N/A".to_string(),
            notes: Default::default(),
            championship: Default::default(),
        }
    }
}
