mod candle;
mod data_loader;
mod analyze;

use crate::candle::{Candle, update_labels};
use crate::data_loader::loader::load_csv;
use std::error::Error;

#[warn(dead_code)]

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, crypto world!");

    
    let path = "BTC_2019_2023_15m.csv";
    let mut candles = load_csv(path)?;
    update_labels(&mut candles);

      for candle in candles.iter().take(10) {
        println!("{:?}", candle);
    }

    
    
    // for candle in candles.iter().take(10) {
    //     println!("{:?}", candle);
    // }
   
   Ok(())

  

}
