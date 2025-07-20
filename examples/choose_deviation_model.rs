use chrono::NaiveDate;
use rust_ti::candle_indicators::bulk::moving_constant_bands;
use rust_ti::ConstantModelType::ExponentialMovingAverage;
use rust_ti::DeviationModel;
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

    let prices: Vec<f64> = data.iter().map(|i| i.close).collect();

    let models = vec![
        DeviationModel::StandardDeviation,
        DeviationModel::MeanAbsoluteDeviation,
        DeviationModel::MedianAbsoluteDeviation,
        DeviationModel::ModeAbsoluteDeviation,
        DeviationModel::UlcerIndex,
    ];

    let mut best_rating = 0.0;
    let mut best_model = DeviationModel::StandardDeviation;

    for m in models.iter() {
        let bands = moving_constant_bands(&prices, ExponentialMovingAverage, *m, 2.0, 5);

        let mut current_rating = 0.0;
        let mut attempt = 0.0;
        for i in 5..data.len() - 1 {
            let upper_band = bands[i - 5].0;
            let lower_band = bands[i - 5].1;

            // If price > upper_band, price is expected to fall, +1 reward if that happens
            if prices[i] > upper_band {
                attempt += 1.0;
                if prices[i + 1] < prices[i] {
                    current_rating += 1.0;
                }
            }

            // If price < lower_band, price is expected to rise, +1 reward if that happens
            if prices[i] < lower_band {
                attempt += 1.0;
                if prices[i + 1] > prices[i] {
                    current_rating += 1.0;
                }
            }
        }

        let average_rating = current_rating / attempt;
        if average_rating > best_rating {
            best_rating = average_rating;
            best_model = *m;
        }
    }

    println!(
        "Best model for moving constant bands is {:?} with a rating of {}",
        best_model, best_rating
    );
}
