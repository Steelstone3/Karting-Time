use iced::widget::{column, Column, Scrollable};
use iced_aw::{TabBar, TabLabel};

use crate::{
    commands::{messages::Message, tab_identifiers::TabIdentifier},
    models::application::karting_time::KartingTime,
};

impl KartingTime {
    pub fn tab_bar_view(&self) -> Column<Message> {
        match self.application_state.tab_identifier {
            TabIdentifier::Race => {
                let tab_bar = selected_tab_bar(&TabIdentifier::Race);

                let contents = Scrollable::new(
                    column!()
                        .push(self.edit_driver_profile_view())
                        .push(self.upsert_race_view()),
                );

                column!(self.menu_bar_view(), tab_bar, contents)
            }
            TabIdentifier::ProfileOverview => {
                let tab_bar = selected_tab_bar(&TabIdentifier::ProfileOverview);

                let contents = Scrollable::new(column!().push(self.overview_driver_profile_view()));

                column!(self.menu_bar_view(), tab_bar, contents)
            }
            TabIdentifier::ResultsOverview => {
                let tab_bar = selected_tab_bar(&TabIdentifier::ResultsOverview);

                let filter: Column<Message> = self.filter_results_view();

                let contents = Scrollable::new(column!().push(self.overview_race_results_view()));

                match self.application_state.is_filter_visible {
                    true => column!(self.menu_bar_view(), tab_bar, filter, contents),
                    false => column!(self.menu_bar_view(), tab_bar, contents),
                }
            }
            TabIdentifier::Results => {
                let tab_bar = selected_tab_bar(&TabIdentifier::Results);

                let filter: Column<Message> = self.filter_results_view();

                let contents = Scrollable::new(column!().push(self.race_results_view()));

                match self.application_state.is_filter_visible {
                    true => column!(self.menu_bar_view(), tab_bar, filter, contents),
                    false => column!(self.menu_bar_view(), tab_bar, contents),
                }
            }
        }
    }
}

fn selected_tab_bar(active_tab: &TabIdentifier) -> TabBar<'static, Message, TabIdentifier> {
    let tab_bar = TabBar::new(Message::SelectedTabChanged)
        .push(
            TabIdentifier::Race,
            TabLabel::IconText('\u{1F3CE}', "Race".to_string()),
        )
        .push(
            TabIdentifier::ProfileOverview,
            TabLabel::IconText('\u{1F464}', "Profile Overview".to_string()),
        )
        .push(
            TabIdentifier::ResultsOverview,
            TabLabel::IconText('\u{1F4CA}', "Results Overview".to_string()),
        )
        .push(
            TabIdentifier::Results,
            TabLabel::IconText('\u{1F3C1}', "Results".to_string()),
        )
        .set_active_tab(active_tab);
    tab_bar
}
