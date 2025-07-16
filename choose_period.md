# How to determine the best period for a RustTI function

This guide shows how to progamatically determine the best period for your indicator.

The rating model is overly simplified and should be refined to suit your needs before usage.

---

## 🎯 Goal

- Determine the best period for the RSI from a year of data

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

The default period for the RSI is 14, we will iterate from 1 to 15 to see if we can find a period that
peforms better than the default.

```rust

[...]

for p in 1..15 {
    let rsi = relative_strenght_index(&prices, SmoothedMovingAverage, p);
}

[...]

```

### 3. Rate each different RSI to find the best

> The logic is overly simple for the purpose of the guide.

If the RSI is over 70 (overbought) and the next price < current price, the period gets a `+1`

If the RSI is under 30 (oversold) and the next price > current price, the period gets a `+1`


```rust

let mut best_rating = 0;
let mut best_period = 0;

for p in 2..15 {
    let rsi = relative_strength_index(&prices, SmoothedMovingAverage, p);
   
    let mut current_rating = 0.0;
    let mut attempt = 0.0;
    for i in p..data.len() - 1 { 
        let rsi_val = rsi[i - p]; 

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
        best_period = p;
    }
}

println!(
    "Best period for RSI is {} with a rating of {}",
    best_period, best_rating
);

[...]

```

---

## 🧪 Output

> to run the repo example `cargo run --example choose_period < data.csv`

```shell
Loaded 251 prices
Best period for RSI is 7 with a rating of 0.5555555555555556
```

---

## ✅ Next Steps

- [Programatically choose a constant type model](./choose_constant_type_model.md) 
- Combine period selection and constant type model selection
- Introduce the notion of punishment to the rating system
