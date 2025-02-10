use crate::{
    commands::messages::Message,
    controllers::files::file_picker::{save_file_location, select_file_to_load},
    models::application::karting_time::KartingTime,
};

impl KartingTime {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::MenuBar => {}
            Message::SelectedTabChanged(tab_identifier) => self.switch_tab(tab_identifier),
            Message::FileNew => self.file_new(),
            Message::ImportDriverProfile => self.import_driver_profile(&select_file_to_load()),
            Message::SaveDriverProfile => self.save_driver_profile(),
            Message::SaveApplication => self.save_application(&save_file_location()),
            Message::LoadApplication => {
                self.load_application(&select_file_to_load());
                self.driver_profile.sort_races();
            }
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
            Message::LaptimeEditor(action) => self
                .application_state
                .race_editor
                .text_editor
                .perform(action),
            Message::UpdateRacesPressed => {
                self.new_race.convert_to_laps(
                    self.application_state
                        .race_editor
                        .get_text_from_text_editor(),
                );
                if self
                    .new_race
                    .is_unique_identifer(&self.driver_profile.races)
                {
                    self.driver_profile.races.push(self.new_race.clone());
                    self.application_state.race_editor.clear_text_editor();
                } else {
                    // TODO Overwrite existing race in "races" vector
                    // TODO confirmation box "Overwrite existing race"
                    self.driver_profile.races = self
                        .new_race
                        .new_race_replaces_existing_race(&self.driver_profile.races);
                }

                self.driver_profile.sort_races()
            }
            Message::ReplacePressed(index) => {
                // TODO Overwrite existing new race in add race section
                if let Some(race) = self.driver_profile.races.get(index) {
                    self.new_race = race.clone();
                    self.application_state.race_editor.clear_text_editor();
                    self.application_state.race_editor.paste_laptimes(race);
                }
            }
        }
    }
}
