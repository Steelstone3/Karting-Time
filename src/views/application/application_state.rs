use crate::{
    commands::{messages::Message, tab_identifier::TabIdentifier},
    models::application::karting_time::KartingTime,
};
use iced::{Element, Theme};

impl KartingTime {
    pub fn view(&self) -> Element<Message> {
        self.tab_bar_view().into()
    }

    pub fn switch_tab(&mut self, tab_identifier: TabIdentifier) {
        match tab_identifier {
            TabIdentifier::Race => {
                self.application_state.tab_identifier = TabIdentifier::Race;
            }
            TabIdentifier::Overview => {
                self.application_state.tab_identifier = TabIdentifier::Overview;
            }
            TabIdentifier::Comparison => {
                self.application_state.tab_identifier = TabIdentifier::Comparison;
            }
            TabIdentifier::Results => {
                self.application_state.tab_identifier = TabIdentifier::Results;
            }
        }
    }

    pub fn theme(&self) -> Theme {
        // Theme::GruvboxDark
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
}
