use chrono::NaiveDate;
use rust_ti::momentum_indicators::bulk::relative_strength_index;
use rust_ti::ConstantModelType;
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
        ConstantModelType::SimpleMovingAverage,
        ConstantModelType::SmoothedMovingAverage,
        ConstantModelType::ExponentialMovingAverage,
        ConstantModelType::SimpleMovingMedian,
        ConstantModelType::SimpleMovingMode,
    ];

    let mut best_rating = 0.0;
    let mut best_model = ConstantModelType::SimpleMovingAverage;

    for m in models.iter() {
        let rsi = relative_strength_index(&prices, *m, 14);

        let mut current_rating = 0.0;
        let mut attempt = 0.0;
        for i in 14..data.len() - 1 {
            let rsi_val = rsi[i - 14];

            // If RSI > 70, overbought, price is expected to fall, if that happens +1 reward
            if rsi_val > 70.0 {
                attempt += 1.0;
                if prices[i + 1] < prices[i] {
                    current_rating += 1.0;
                }
            }

            // IF RSI < 30, oversold, price is expected to rise, if that happens +1 reward
            if rsi_val < 30.0 {
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
        "Best model for RSI is {:?} with a rating of {}",
        best_model, best_rating
    );
}
