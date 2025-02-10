use crate::{
    commands::tab_identifier::TabIdentifier, models::driver_results::race_editor::RaceEditor,
};
use serde::{Deserialize, Serialize};

#[derive(Default, PartialEq, Serialize, Deserialize, Debug)]
pub struct ApplicationState {
    pub tab_identifier: TabIdentifier,
    pub is_light_theme: bool,
    #[serde(skip)]
    pub race_editor: RaceEditor,
}
