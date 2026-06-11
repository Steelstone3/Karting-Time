#[derive(Debug, Clone, PartialEq, Default)]
pub struct Lap {
    pub lap_number: u32,
    pub time: f32,
}

impl Lap {
    pub fn new(lap_number: u32, time: f32) -> Self {
        Self { lap_number, time }
    }
}
