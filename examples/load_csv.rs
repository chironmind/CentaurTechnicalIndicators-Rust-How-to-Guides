use chrono::NaiveDate;
use rust_ti::momentum_indicators::bulk::relative_strength_index;
use rust_ti::ConstantModelType::SmoothedMovingAverage;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct Ohlc {
    #[serde(with = "csv_date_format")]
    date: NaiveDate,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

mod csv_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &'static str = "%m/%d/%Y";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

fn get_data() -> Vec<Ohlc> {
    let mut prices = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for line in rdr.deserialize() {
        let ohlc: Ohlc = line.expect("");
        prices.push(ohlc);
    }
    return prices;
}

fn main() {
    let data = get_data();
    println!("Loaded {} prices", data.len());

    let close: Vec<f64> = data.iter().map(|i| i.close).collect();
    let rsi = relative_strength_index(&close, SmoothedMovingAverage, 14);
    println!("Calculated {} rsis", rsi.len());
}
