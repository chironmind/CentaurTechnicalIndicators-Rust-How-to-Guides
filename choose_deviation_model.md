# How to determine the best `DeviationModel` for a RustTI function

This guide shows how to progamatically determine the best `DeviationModel` for your indicator.

The rating model is overly simplified and should be refined to suit your needs before usage.

---

## 🎯 Goal

- Determine the best `DeviationModel` for the Moving Constant Bands (generic Bollinger bands) from a year of data

> This guide uses knowledge established in the [load csv](./load_csv.md) guide.

---

## 📦 Requirements

Add the following dependencies to your Cargo.toml:

```toml
[dependencies]
csv = "1"
serde = { version = "1", features = ["derive"] }
chrono = "0.4"
rust_ti = "2.1"
```

---

## 💻 Step-by-Step

### 1. Get data from CSV

[See load csv guide](./load_csv.md) if you need a refresher here.

### 2. Calculate the moving constant bands for multiple deviation models

The default deviation model to use is the standard deviation, however other models may provide more insight.

We will store the models we are interested in into a Vec and iterate over it to calculate the bands.

```rust

use rust_ti::DeviationModel;
use rust_ti::ConstantModelType::ExponentialMovingAverage;
use rust_ti::candle_indicators::bulk::moving_constant_bands;

[...]

let models = vec![
    DeviationModel::StandardDeviation,
    DeviationModel::MeanAbsoluteDeviation,
    DeviationModel::MedianAbsoluteDeviation,
    DeviationModel::ModeAbsoluteDeviation,
    DeviationModel::UlcerIndex,
];

for m in models.iter() {
    let bands = moving_constant_bands(&prices, ExponentialMovingAverage, *m, 2.0, 5);
}

[...]

```

### 3. Rate each different constant bands model to find the best

> The logic is overly simple for the purpose of the guide.

If price > upper band (overbought) and next price < current price, model gets a `+1`

If price < lower band (oversold) and next price > current price, model gets a `+1`

```rust

[...]

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
        if price[i] > upper_band {
            attempt += 1.0;
            if prices[i + 1] < prices[i] {
                current_rating += 1.0;
            }
        }

        // If price < lower_band, price is expected to rise, +1 reward if that happens
        if price[i] < lower_band {
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

[...]

```

---

## 🧪 Output

> to run the repo example `cargo run --example choose_deviation_model < data.csv`

```shell
Loaded 251 prices
Best model for moving constant bands is MedianAbsoluteDeviation with a rating of 0.47333333333333333
```

---

## ✅ Next Steps

- Programatically choose a period
- Programatically choose a `ConstantModelType`
- Programatically choose a deviation multiplier
- Combine all selections
- Introduce the notion of punishment to the rating system

