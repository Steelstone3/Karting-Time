use super::tab_identifiers::TabIdentifier;
use iced::{advanced::widget::operation::scrollable, widget::text_editor};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    MenuBar,
    FileNew,
    ImportRaces,
    ExportRaces,
    SaveApplication,
    LoadApplication,
    ViewToggleTheme,
    ViewToggleFilter,
    SelectedTabChanged(TabIdentifier),
    DriverNameChanged(String),
    TrackNameChanged(String),
    DayChanged(String),
    MonthChanged(String),
    YearChanged(String),
    SessionIdChanged(String),
    RacePositionChanged(String),
    CarUsedChanged(String),
    LaptimeEditor(text_editor::Action),
    TrackFilterChanged(String),
    DateFilterChanged(String),
    ReplacePressed(usize),
    UpdateRacesPressed,
    ClearRaceEditorPressed,
    SyncHeader(scrollable::AbsoluteOffset),
}
