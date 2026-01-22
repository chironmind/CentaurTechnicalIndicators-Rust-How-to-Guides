# How to determine the best `ConstantModelType` for a CentaurTechnicalIndicators-Rust function

This guide shows how to programmatically determine the best `ConstantModelType` for your indicator.

The rating model is overly simplified and should be refined to suit your needs before usage.

---

## 🎯 Goal

- Determine the best `ConstantModelType` for the [RSI](https://docs.rs/centaur_technical_indicators/latest/centaur_technical_indicators/momentum_indicators/bulk/fn.relative_strength_index.html) from a year of data

> This guide uses knowledge established in the [load csv](./load_csv.md) guide.

---

## 📦 Requirements

Add the following dependencies to your Cargo.toml:

```toml
[dependencies]
csv = "1"
serde = { version = "1", features = ["derive"] }
chrono = "0.4"
centaur_technical_indicators = "1.0"
```

---

## 💻 Step-by-Step

### 1. Get data from CSV

[See load csv guide](./load_csv.md) if you need a refresher here.

### 2. Calculate the RSI for multiple periods

The default model for the RSI is a Smoothed Moving Average (SMA)

We will store the models we are interested in into a Vec and iterate over it to calculate the RSI.

In this guide we will skip the Personalised Moving Average as it is covered in [another guide](./personalised_moving_average.md)

```rust
use centaur_technical_indicators::ConstantModelType;

// [...]

fn main() {
    let data = get_data();
    
    let models = vec![
        ConstantModelType::SimpleMovingAverage, ConstantModelType::SmoothedMovingAverage,
        ConstantModelType::ExponentialMovingAverage, ConstantModelType::SimpleMovingMedian,
        ConstantModelType::SimpleMovingMode
    ];

    for m in models.iter() {
        let rsi = relative_strength_index(&prices, *m, 14);
    }
}
```

### 3. Rate each different RSI to find the best

> The logic is overly simplified for the purpose of the guide.

If the RSI is over 70 (overbought) and the next price < current price, the model gets a `+1`

If the RSI is under 30 (oversold) and the next price > current price, the model gets a `+1`


```rust
// [...]

fn main() {
    let data = get_data();

    let models = vec![
        ConstantModelType::SimpleMovingAverage, ConstantModelType::SmoothedMovingAverage,
        ConstantModelType::ExponentialMovingAverage, ConstantModelType::SimpleMovingMedian,
        ConstantModelType::SimpleMovingMode
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

            // If RSI < 30, oversold, price is expected to rise, if that happens +1 reward
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

```

---

## 🧪 Output

The full code for this guide can be found in [`./examples/choose_constant_model_type.rs`](./examples/bulk_vs_single.rs).

To run it:
```bash
cd examples
cargo run --example choose_constant_model_type < data.csv
```
Expected output:
```shell
Loaded 251 prices
Best model for RSI is SimpleMovingAverage with a rating of 0.5625
```

---

## ✅ Next Steps

- [Programmatically choose a deviation model](./choose_deviation_model.md)
- [Programmatically choose a period](./choose_period.md) 
