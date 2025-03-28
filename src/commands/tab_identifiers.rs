use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabIdentifier {
    Race,
    #[default]
    ProfileOverview,
    ResultsOverview,
    Results,
}
