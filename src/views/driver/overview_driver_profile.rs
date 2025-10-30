use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::{
    Element,
    widget::{Column, column, text},
};
use iced_aw::Card;
use iced_table::Table;

impl KartingTime {
    pub fn overview_driver_profile_view(&self) -> Column<'_, Message> {
        let content = column!()
            .push(text(self.driver_profile.name.to_string()).size(24))
            .padding(10)
            .spacing(10)
            .push(self.overview_driver_profile_table());

        column!()
            .push(Card::new(
                text(self.driver_profile.name.to_string()),
                content,
            ))
            .padding(10)
            .spacing(10)
    }

    fn overview_driver_profile_table(&self) -> Element<'_, Message> {
        let mut table = Table::default();

        table.add_headers(vec!["Profile Summary", "Driver Statistic"]);

        table.add_rows(vec![
            vec![
                "Races",
                &self.driver_profile.profile_statistics.races.to_string(),
            ],
            vec![
                "Wins",
                &self.driver_profile.profile_statistics.wins.to_string(),
            ],
            vec![
                "Podiums",
                &self.driver_profile.profile_statistics.podiums.to_string(),
            ],
            vec![
                "Top Fives",
                &self.driver_profile.profile_statistics.top_5.to_string(),
            ],
            vec![
                "Top Tens",
                &self.driver_profile.profile_statistics.top_10.to_string(),
            ],
            vec![
                "Unique Tracks",
                &self
                    .driver_profile
                    .profile_statistics
                    .unique_tracks
                    .to_string(),
            ],
            vec![
                "Unique Cars",
                &self
                    .driver_profile
                    .profile_statistics
                    .unique_cars
                    .to_string(),
            ],
        ]);

        Table::build(
            table,
            Some(self.theme().palette().text),
            Some(300.0),
            None,
            None,
        )
    }
}
