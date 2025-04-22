use super::race_editor::RaceEditor;
use crate::{commands::tab_identifiers::TabIdentifier, models::driver::race_result::Race};
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApplicationState {
    pub tab_identifier: TabIdentifier,
    pub is_light_theme: bool,
    #[serde(skip)]
    pub race_editor: RaceEditor,
    #[serde(skip)]
    pub new_race: Race,
    #[serde(skip)]
    pub search_query: String,
    #[serde(skip)]
    pub filtered_races: Vec<Race>,
}
