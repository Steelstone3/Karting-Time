use iced::{
    Border, Color, Element, Length, Renderer, Settings, Theme,
    border::Radius,
    widget::{Column, Container, Row, container, text},
};

use crate::commands::messages::Message;

const TEXT_SIZE: u16 = 16;

#[derive(Default)]
pub struct Table {
    pub rows: Vec<Vec<String>>,
}

impl Table {
    #[allow(dead_code)]
    pub fn new(headers: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        let mut new_rows: Vec<Vec<String>> = vec![];
        new_rows.push(headers);
        for row in rows {
            new_rows.push(row);
        }

        Self { rows: new_rows }
    }

    pub fn add_headers(&mut self, headers: Vec<String>) {
        self.rows.insert(0, headers);
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        self.rows.push(row);
    }

    pub fn build(table_data: Table, width: Option<f32>) -> Element<'static, Message> {
        let mut table = Column::new();

        // with header and rows
        for row in table_data.rows {
            let row = Table::with_row(row);

            table = table.push(row);
        }

        match width {
            Some(width) => Container::new(table)
                .width(Length::Fixed(width))
                .padding(10)
                .into(),
            None => Container::new(table).width(Length::Fill).padding(10).into(),
        }
    }

    fn with_row(rows: Vec<String>) -> Row<'static, Message> {
        let mut data_row = vec![];
        for row in rows.into_iter() {
            data_row.push(
                Container::<Message, Theme, Renderer>::new(text(row).size(TEXT_SIZE))
                    .padding(2)
                    .width(Length::Fill)
                    // .style(|theme: &Theme| container::Style {
                    //     border: Border {
                    //         color: Color::from_rgb(0.0, 97.0, 97.0),
                    //         width: 5.0,
                    //         radius: Radius::new(5),
                    //     },
                    //     ..Default::default() // background: theme.extended_palette().background.base.color.into(), // Default background
                    //                          // text_color: theme.extended_palette().text.primary, // Default text color
                    //                          // shadow: todo!(),
                    // })
                    // .style(|theme: &Theme| container::Style {
                    //     // background: , // Apply custom background
                    //     ..Default::default()
                    // })
                    .into(),
            );
        }

        // build the data row
        Row::with_children(data_row)
    }

    #[allow(dead_code)]
    pub fn calculate_width(table_data: &Table) -> f32 {
        if table_data.rows.is_empty() {
            return 0.0;
        }

        let text_size = TEXT_SIZE as f32;
        let mut rows_widths: Vec<Vec<usize>> = vec![];

        for row in &table_data.rows {
            let mut cell_widths = vec![];

            for cell in row {
                cell_widths.push(cell.len());
            }

            rows_widths.push(cell_widths);
        }

        let mut max_row_width = 0.0;

        for row_widths in rows_widths {
            let mut new_row_width = 0.0;

            for row_width in row_widths {
                new_row_width += row_width as f32 * text_size;
            }

            if new_row_width > max_row_width {
                max_row_width = new_row_width;
            }
        }

        max_row_width
    }
}
