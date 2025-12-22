use crate::models::driver::session_information::race_result::RaceResult;
use iced::widget::text_editor::{self, Action, Content, Edit};

#[derive(Default, Debug)]
pub struct RaceEditor {
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
        self.text_editor = Content::new();
    }

    pub fn paste_laptimes(&mut self, race: &RaceResult) {
        self.text_editor.perform(Action::Edit(Edit::Paste(
            race.convert_laps_to_string().into(),
        )));
    }
}

#[cfg(test)]
mod race_editor_should {
    use crate::models::driver::session_information::lap::Lap;

    use super::*;

    #[test]
    fn clone() {
        // Given
        let expected_race_editor = RaceEditor {
            text_editor: Default::default(),
        };

        // When
        let race_editor = expected_race_editor.clone();

        // Then
        pretty_assertions::assert_eq!(expected_race_editor, race_editor)
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
        pretty_assertions::assert_eq!(expected_text, text)
    }

    #[test]
    pub fn clear_text_editor() {
        // Given
        let expected_text = "";
        let mut race_editor = RaceEditor {
            text_editor: Default::default(),
        };
        race_editor
            .text_editor
            .perform(Action::Edit(Edit::Paste("Hello There".to_string().into())));

        // When
        race_editor.clear_text_editor();

        // Then
        pretty_assertions::assert_eq!(expected_text, race_editor.text_editor.text())
    }

    #[test]
    fn paste_laptimes() {
        // Given
        let race = RaceResult::new(
            Default::default(),
            Default::default(),
            vec![Lap::new(1, 40.965), Lap::new(2, 41.875)],
        );
        let mut race_editor = RaceEditor {
            text_editor: Default::default(),
        };

        // When
        race_editor.paste_laptimes(&race);

        // Then
        pretty_assertions::assert_eq!("40.965\n41.875\n", race_editor.text_editor.text())
    }
}
