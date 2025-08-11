use chrono::NaiveDateTime;


pub struct Candle {
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub open_time: String,
}

impl Candle {

    pub fn new(open: f64, close: f64, high: f64, low: f64, open_time: String) -> Self {
        Self { open, close, high, low, open_time}
    }   
}

