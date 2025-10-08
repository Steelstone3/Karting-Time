use crate::commands::messages::Message;
use crate::models::application::karting_time::KartingTime;
use iced::Length;
use iced::widget::{button, column};
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items};

impl KartingTime {
    pub fn menu_bar_view(&self) -> iced::widget::Column<'_, Message> {
        let menu_template = |items| Menu::new(items).max_width(180.0).offset(6.0);

        let menu_bar = menu_bar!((
            button("File").on_press(Message::MenuBar),
            menu_template(menu_items!((button("New")
                .width(Length::Fill)
                .on_press(Message::FileNew))(
                button("Save Driver Profile")
                    .width(Length::Fill)
                    .on_press(Message::SaveApplicationRequested)
            )(
                button("Load Driver Profile")
                    .width(Length::Fill)
                    .on_press(Message::LoadApplicationRequested)
            )(
                button("Import Race")
                    .width(Length::Fill)
                    .on_press(Message::ImportRacesRequested)
            )(
                button("Export Races")
                    .width(Length::Fill)
                    .on_press(Message::ExportRacesRequested)
            )(
                button("Export HTML Races")
                    .width(Length::Fill)
                    .on_press(Message::ExportHtmlRacesRequested)
            )))
        )(
            button("View").on_press(Message::MenuBar),
            menu_template(menu_items!((button("Toggle Theme")
                .width(Length::Fill)
                .on_press(Message::ViewToggleTheme))(
                button("Toggle Filter")
                    .width(Length::Fill)
                    .on_press(Message::ViewToggleFilter)
            )))
        ));

        column!().push(menu_bar)
    }
}
