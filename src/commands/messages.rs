use super::tab_identifier::TabIdentifier;
use iced::widget::text_editor;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    MenuBar,
    FileNew,
    ImportRaces,
    ExportRaces,
    SaveApplication,
    LoadApplication,
    ViewToggleTheme,
    SelectedTabChanged(TabIdentifier),
    DriverNameChanged(String),
    TrackNameChanged(String),
    DayChanged(String),
    MonthChanged(String),
    YearChanged(String),
    SessionIdChanged(String),
    RacePositionChanged(String),
    LaptimeEditor(text_editor::Action),
    ReplacePressed(usize),
    UpdateRacesPressed,
    ClearRaceEditorPressed,
}
