use crate::commands::tab_identifier::TabIdentifier;
use iced::widget::text_editor::{self, Action, Edit};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct ApplicationState {
    pub tab_identifier: TabIdentifier,
    pub is_light_theme: bool,
    #[serde(skip)]
    pub text_editor: text_editor::Content,
}

impl PartialEq for ApplicationState {
    fn eq(&self, other: &Self) -> bool {
        self.tab_identifier == other.tab_identifier && self.is_light_theme == other.is_light_theme
    }
}

impl ApplicationState {
    pub fn get_text_from_text_editor(&self) -> String {
        self.text_editor.text()
    }
    pub fn clear_text_editor(&mut self) {
        self.text_editor.perform(Action::SelectAll);

        self.text_editor
            .perform(Action::Edit(Edit::Paste("".to_string().into())));
    }
}
