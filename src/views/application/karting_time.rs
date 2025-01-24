use crate::{commands::messages::Message, models::application::karting_time::KartingTime};

impl KartingTime {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::MenuBar => {}
            Message::SelectedTabChanged(tab_identifier) => self.switch_tab(tab_identifier),
            Message::FileNew => self.file_new(),
            // TODO File importer
            Message::FileImport => {}
            Message::FileSave => self.file_save(),
            Message::FileLoad => self.file_load(),
            Message::ViewToggleTheme => self.switch_theme(),
        }
    }
}
