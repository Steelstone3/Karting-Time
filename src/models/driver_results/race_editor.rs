use iced::widget::text_editor::{self, Action, Edit};
use serde::{Deserialize, Serialize};

use super::race_result::Race;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct RaceEditor {
    #[serde(skip)]
    pub text_editor: text_editor::Content,
}

impl Clone for RaceEditor {
    fn clone(&self) -> Self {
        Self {
            text_editor: Default::default(),
        }
    }
}

impl PartialEq for RaceEditor {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl RaceEditor {
    pub fn get_text_from_text_editor(&self) -> String {
        self.text_editor.text()
    }

    pub fn clear_text_editor(&mut self) {
        self.text_editor.perform(Action::SelectAll);

        self.text_editor
            .perform(Action::Edit(Edit::Paste("".to_string().into())));
    }

    pub fn paste_laptimes(&mut self, race: &Race) {
        self.text_editor.perform(Action::Edit(Edit::Paste(
            race.convert_laps_to_string().into(),
        )));
    }
}
