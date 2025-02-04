use crate::commands::tab_identifier::TabIdentifier;
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApplicationState {
    pub tab_identifier: TabIdentifier,
    pub is_light_theme: bool,
    #[serde(skip)]
    pub is_editing_races: bool,
}
