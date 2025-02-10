use crate::models::date::Date;

use super::{race_information::RaceInformation, race_result::Race};

pub struct RaceFile {
    pub race_information: RaceInformation,
    pub laptimes: Vec<f32>,
}

impl RaceFile {

}