use crate::candle::Candle;
use csv::ReaderBuilder;
use std::error::Error;

pub fn load_csv(path: &str) -> Result<Vec<Candle>, Box<dyn Error>> {

    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;

    let mut candles = Vec::new();

    for result in rdr.deserialize() {
        let record: Candle = result?;
        candles.push(record);
    }
    Ok(candles)
    
}