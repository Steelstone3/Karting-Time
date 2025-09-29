use super::race_editor::RaceEditor;
use crate::commands::tab_identifiers::TabIdentifier;

#[derive(Default, PartialEq, Debug)]
pub struct ApplicationState {
    pub tab_identifier: TabIdentifier,
    pub is_light_theme: bool,
    pub race_editor: RaceEditor,
}
