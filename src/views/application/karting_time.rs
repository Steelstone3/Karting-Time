use crate::{commands::messages::Message, models::application::karting_time::KartingTime};

impl KartingTime {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::MenuBar => {}
            Message::SelectedTabChanged(tab_identifier) => self.switch_tab(tab_identifier),
            Message::FileNew => self.file_new(),
            Message::LoadDriverProfile => self.load_driver_profile(),
            Message::SaveDriverProfile => self.save_driver_profile(),
            Message::SaveApplication => self.save_application(),
            Message::LoadApplication => self.load_application(),
            Message::ViewToggleTheme => self.switch_theme(),
            Message::DriverNameChanged(name) => self.driver_profile.name = name,
            Message::TrackNameChanged(track_name) => self.new_race.track_name = track_name,
            Message::DayChanged(day) => {
                self.new_race.date.set_day(day);
            }
            Message::MonthChanged(month) => {
                self.new_race.date.set_month(month);
            }
            Message::YearChanged(year) => {
                self.new_race.date.set_year(year);
            }
            Message::SessionIdChanged(session_id) => {
                self.new_race.update_session_id(session_id);
            }
            Message::RacePositionChanged(race_position) => {
                self.new_race.update_race_position(race_position);
            }
            Message::LaptimeEditor(action) => self.application_state.text_editor.perform(action),
            Message::AddRacePressed => {
                self.new_race
                    .convert_to_laps(self.application_state.get_text_from_text_editor());
                self.driver_profile.races.push(self.new_race.clone());
                self.application_state.clear_text_editor();
            }
        }
    }
}
