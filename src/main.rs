mod candle;
mod data_loader;
mod analyze;

use std::error::Error;

use crate::data_loader::loader::load_csv;
use crate::candle::Candle;
use crate::analyze::analyzer;
#[warn(dead_code)]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, crypto world!");

    
    let path = "BTC_2019_2023_15m.csv";
    let candles = load_csv(path)?;

   Ok(())

  

}
