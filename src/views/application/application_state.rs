use crate::{
    commands::{messages::Message, tab_identifiers::TabIdentifier},
    models::application::karting_time::KartingTime,
};
use iced::{Element, Task, Theme};

impl KartingTime {
    pub fn boot() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn view(&self) -> Element<'_, Message> {
        self.tab_bar_view().into()
    }

    pub fn switch_tab(&mut self, tab_identifier: TabIdentifier) {
        match tab_identifier {
            TabIdentifier::Race => {
                self.application_state.tab_identifier = TabIdentifier::Race;
            }
            TabIdentifier::ProfileOverview => {
                self.application_state.tab_identifier = TabIdentifier::ProfileOverview;
            }
            TabIdentifier::ResultsOverview => {
                self.application_state.tab_identifier = TabIdentifier::ResultsOverview
            }
            TabIdentifier::Results => {
                self.application_state.tab_identifier = TabIdentifier::Results;
            }
        }
    }

    pub fn theme(&self) -> Theme {
        match self.application_state.is_light_theme {
            true => Theme::Light,
            false => Theme::Dark,
        }
    }

    pub fn switch_theme(&mut self) {
        match self.application_state.is_light_theme {
            true => self.application_state.is_light_theme = false,
            false => self.application_state.is_light_theme = true,
        }
    }

    pub fn toggle_filter(&mut self) {
        match self.driver_profile.filter.is_filter_visible {
            true => self.driver_profile.filter.is_filter_visible = false,
            false => self.driver_profile.filter.is_filter_visible = true,
        }
    }
}
