use crate::commands::messages::Message;
use crate::models::application::karting_time::KartingTime;
use iced::widget::{button, column, row, text};
use iced::{Length, alignment};

impl KartingTime {
    pub fn pagination_bar_view(&self) -> iced::widget::Column<'_, Message> {
        let pagination = row!(
            button(
                text("←")
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center)
            )
            .width(Length::Fill)
            .on_press(Message::PaginationPrevious),
            text(self.driver_profile.filter.page_of_total_pages()),
            button(
                text("→")
                    .align_x(alignment::Horizontal::Center)
                    .align_y(alignment::Vertical::Center)
            )
            .width(Length::Fill)
            .on_press(Message::PaginationNext)
        )
        .spacing(10);

        column!(pagination)
    }
}
