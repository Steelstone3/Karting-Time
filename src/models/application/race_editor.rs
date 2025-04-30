use crate::models::driver::race_result::Race;
use iced::widget::text_editor::{self, Action, Edit};
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod race_editor_should {
    use super::*;
    use crate::models::driver::lap::Lap;

    #[test]
    fn clone() {
        // Given
        let expected_race_editor = RaceEditor {
            text_editor: Default::default(),
        };

        // When
        let race_editor = expected_race_editor.clone();

        // Then
        assert_eq!(expected_race_editor, race_editor)
    }

    #[test]
    fn get_text_from_text_editor() {
        // Given
        let expected_text = "Hello There".to_string();
        let mut race_editor = RaceEditor {
            text_editor: Default::default(),
        };
        race_editor
            .text_editor
            .perform(Action::Edit(Edit::Paste(expected_text.to_string().into())));

        // When
        let text = race_editor.get_text_from_text_editor();

        // Then
        assert_eq!(expected_text + "\n", text)
    }

    #[test]
    pub fn clear_text_editor() {
        // Given
        let expected_text = "\n";
        let mut race_editor = RaceEditor {
            text_editor: Default::default(),
        };
        race_editor
            .text_editor
            .perform(Action::Edit(Edit::Paste("Hello There".to_string().into())));

        // When
        race_editor.clear_text_editor();

        // Then
        assert_eq!(expected_text, race_editor.text_editor.text())
    }

    #[test]
    fn paste_laptimes() {
        // Given
        let race = Race {
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 40.965,
                },
                Lap {
                    lap_number: 2,
                    time: 41.875,
                },
            ],
            ..Default::default()
        };

        let mut race_editor = RaceEditor {
            text_editor: Default::default(),
        };
        // When
        race_editor.paste_laptimes(&race);

        // Then
        assert_eq!("40.965\n41.875\n", race_editor.text_editor.text())
    }
}
