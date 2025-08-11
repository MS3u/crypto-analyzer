mod candle;
use crate::candle::Candle;
fn main() {
    println!("Hello, crypto world!");

    let test_candle = Candle {
        open: 100.0,
        close: 150.0,
        high: 160.0,
        low: 95.0,
        open_time: String::from("2025-08-11 10:00:00")

    };
    
}
