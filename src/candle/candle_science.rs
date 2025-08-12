use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Candle {
    #[serde(rename = "")]
    pub _index: Option<u32>,
    pub datetime: String,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
}

// impl Candle {

//     pub fn new(open: f64, close: f64, high: f64, low: f64, open_time: String, volume: f64) -> Self {
//         Self { open, close, high, low, open_time, volume }
//     }   
// }

