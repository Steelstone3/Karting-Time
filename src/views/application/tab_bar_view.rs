use iced::widget::{column, Column, Scrollable};
use iced_aw::{TabBar, TabLabel};

use crate::{
    commands::{messages::Message, tab_identifier::TabIdentifier},
    models::application::karting_time::KartingTime,
};

impl KartingTime {
    pub fn tab_bar_view(&self) -> Column<Message> {
        match self.application_state.tab_identifier {
            TabIdentifier::Race => {
                let tab_bar = selected_tab_bar(&TabIdentifier::Race);

                let contents = Scrollable::new(column!().push(self.race_view()));

                column!(self.menu_view(), tab_bar, contents)
            }
            TabIdentifier::Overview => {
                let tab_bar = selected_tab_bar(&TabIdentifier::Overview);

                let contents = Scrollable::new(column!().push(self.overview_view()));

                column!(self.menu_view(), tab_bar, contents)
            }
            TabIdentifier::Results => {
                let tab_bar = selected_tab_bar(&TabIdentifier::Results);

                let contents = Scrollable::new(column!().push(self.results_view()));

                column!(self.menu_view(), tab_bar, contents)
            }
            TabIdentifier::Comparison => {
                let tab_bar = selected_tab_bar(&TabIdentifier::Comparison);

                // TODO comparison view
                let contents = Scrollable::new(column!().push(column![])); //self.information_view()

                column!(self.menu_view(), tab_bar, contents)
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
            TabIdentifier::Overview,
            TabLabel::IconText('\u{1F464}', "Overview".to_string()),
        )
        .push(
            TabIdentifier::Results,
            TabLabel::IconText('\u{1F4CA}', "Results".to_string()),
        )
        .push(
            TabIdentifier::Comparison,
            TabLabel::IconText('\u{1F50D}', "Comparison".to_string()),
        )
        .set_active_tab(active_tab);
    tab_bar
}
