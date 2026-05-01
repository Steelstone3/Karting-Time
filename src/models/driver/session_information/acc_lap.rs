use serde::Deserialize;

 // TODO Move to models
    #[derive(Clone, Default, Debug, Deserialize)]
    pub struct AccLap {
        // TODO May want to consider grouping laptimes per driver index and making separate enteries for each
        // pub driverIndex: u32,
        pub laptime: f32,
    }