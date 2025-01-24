use crate::commands::tab_identifier::TabIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationState {
    pub tab_identifier: TabIdentifier,
    pub is_light_theme: bool,
}

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            tab_identifier: TabIdentifier::Overview,
            is_light_theme: Default::default(),
        }
    }
}
