use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TabIdentifier {
    AddRace,
    Overview,
    Results,
    Comparison,
}
