use chrono::NaiveDate;
use rust_ti::candle_indicators::bulk::mcginley_dynamic_bands as bulk_md_bands;
use rust_ti::candle_indicators::single::mcginley_dynamic_bands as single_md_bands;
use rust_ti::DeviationModel::StandardDeviation;
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

    let mut prices: Vec<f64> = data.iter().map(|i| i.close).collect();

    let bands = bulk_md_bands(&prices, StandardDeviation, 2.0, 0.0, 5);

    println!("Length of bands {}", bands.len());

    // Next prices comes in
    prices.push(5689.24);

    let next_band = single_md_bands(&prices[247..], StandardDeviation, 2.0, bands.last().unwrap().1);

    println!(
        "Lower band {}, McGinley dynamic {}, upper band {}",
        next_band.0, next_band.1, next_band.2
    );
}
