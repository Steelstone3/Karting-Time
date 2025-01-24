use super::tab_identifier::TabIdentifier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    MenuBar,
    FileNew,
    FileImport,
    FileSave,
    FileLoad,
    ViewToggleTheme,
    SelectedTabChanged(TabIdentifier),
}
