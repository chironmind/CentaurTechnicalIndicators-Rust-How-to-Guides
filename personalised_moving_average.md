# How to determine the best `ConstantModelType::PersonalisedMovingAverage` for a RustTI function

The personalised moving average is a generic version of the math behind the smooth and exponential moving averages.

Both models add heavier weights to more recent prices. The weight is determined by the alpha, which the caller can
influence by passing in an alpha numerator and an alpha denomintator to `MovingAverageType::Personalised`.

For the smoothed moving average `aplha = 1 / prices.len()`

For the exponential moving average `alpha = 2 / (prices.len() + 1)`

This guide shows how to progamatically determine the best `ConstantModelType::PersonalisedMovingAverage` for your indicator.

The rating model is overly simplified and should be refined to suit your needs before usage.

---

## 🎯 Goal

- Determine the best `ConstantModelType::PersonalisedMovingAverage` for the RSI from a year of data

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

### 2. Calculate the RSI for multiple periods

We will iterate from 0.0 to 5.0, with a step of 1. 
As `ConstantModelType::PersonalisedMovingAverage` takes a float you could you have a smaller step for more precision.

```rust

use rust_ti::ConstantModelType;

[...]

for num in 0.0..=5.0 {
    for denom in 0.0..=5.0 {
        let rsi = relative_strenght_index(
            &prices, 
            ConstantModelType::PersonalisedMovingAverage{ alpha_num: num, alpha_denom: denom},
            14
        );
    }
}

[...]

```

### 3. Rate each different RSI to find the best

> The logic is overly simple for the purpose of the guide.

If the RSI is over 70 (overbought) and the next price < current price, the model gets a `+1`

If the RSI is under 30 (oversold) and the next price > current price, the model gets a `+1`


```rust

let mut best_rating = 0.0;
let mut best_numerator = 0.0;
let mut best_denominator = 0.0;

for num in 0..=5 {
    for denom in 0..=5 {
        let num = num as f64;
        let denom = denom as f64;
        let rsi = relative_strength_index(
            &prices, 
            ConstantModelType::PersonalisedMovingAverage{ 
                alpha_num: num, 
                alpha_den: denom
            },
            14
        );
       
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
        // The shorter the period the more RSIs, so the more opportunities to be right,
        // for fairness we divide by the number of attempts
        let average_rating = current_rating / attempt;
        if average_rating > best_rating {
            best_rating = average_rating;
            best_numerator = num;
            best_denominator = denom;
        }
    }
}

println!(
    "Best numerator {}, best denominator {} for RSI, with a rating of {}",
    best_numerator, best_denominator, best_rating
);

[...]

```

---

## 🧪 Output

> to run the repo example `cargo run --example personalised_moving_average < data.csv`

```shell
Loaded 251 prices
Best numerator 5, best denominator 1 for RSI, with a rating of 0.6129032258064516
```

---

## ✅ Next Steps

- [Programatically choose a period](./choose_period.md) 
- Combine period selection and constant type model selection
- Introduce the notion of punishment to the rating system
