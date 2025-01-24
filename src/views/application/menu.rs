use crate::commands::messages::Message;
use crate::models::application::karting_time::KartingTime;
use iced::widget::{button, column};
use iced::Length;
use iced_aw::menu::{Item, Menu};
use iced_aw::{menu_bar, menu_items};

impl KartingTime {
    pub fn menu_view(&self) -> iced::widget::Column<Message> {
        let menu_template = |items| Menu::new(items).max_width(180.0).offset(6.0);

        let menu_bar = menu_bar!((
            button("File").on_press(Message::MenuBar),
            menu_template(menu_items!((button("New")
                .width(Length::Fill)
                .on_press(Message::FileNew))(
                button("Import")
                    .width(Length::Fill)
                    .on_press(Message::FileImport)
            )(
                button("Save")
                    .width(Length::Fill)
                    .on_press(Message::FileSave)
            )(
                button("Load")
                    .width(Length::Fill)
                    .on_press(Message::FileLoad)
            )))
        )(
            button("View").on_press(Message::MenuBar),
            menu_template(menu_items!(
                (button("Toggle Theme")
                    .width(Length::Fill)
                    .on_press(Message::ViewToggleTheme))
            ))
        ));

        column!().push(menu_bar)
    }
}
