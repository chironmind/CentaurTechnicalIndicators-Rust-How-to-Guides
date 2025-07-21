# When to choose bulk vs single functions in RustTI

This guide shows when to choose the bulk version of a function or the single version of a function.

---

## 🎯 Goal

- Understand when to use bulk or single functions

---

## 📦 Requirements

Add the following dependencies to your Cargo.toml:

```toml
[dependencies]
rust_ti = "2.1"
```

---

## 💻 Step-by-Step

### 1. What is the period you're looking at?

Determine the amount of time data you want to include when computing the indicator.

### 2. Observe the data

Do you have just enough data to cover the period? 

If yes, then you will only be able to calculate a single value for the indicator

Do you have extra data? 

If yes, then you will be able to calculate mutliple values for the indicator

### 3. What is your goal?

Will previous values for the indicator help you make a decision?

If yes, make sure you collect enough data to be able to calculate previous values

### 4. Example

The default RSI takes 14 previous prices, if you have 53 days worth of previous data you can 
use the bulk function to calculate the previous RSIs.

When a new price comes in you can calculate the next RSI with the single function.

```rust

use rust_ti::momentum_indicators::bulk::relative_strength_index as bulk_rsi;
use rust_ti::momentum_indicators::single::relative_strength_index as single_rsi;
use rust_ti::ConstantModelType::SmoothedMovingAverage;

fn main() {
    let mut data = vec![
        6037.59, 5970.84, 5906.94, 5881.63, 5868.55, 5942.47, 5975.38, 5909.03,
        5918.25, 5827.04, 5836.22, 5842.91, 5949.91, 5937.34, 5996.66, 6049.24,
        6086.37, 6118.71, 6101.24, 6012.28, 6067.70, 6039.31, 6071.17, 6040.53,
        5994.57, 6037.88, 6061.48, 6083.57, 6025.99, 6066.44, 6068.50, 6051.97,
        6115.07, 6114.63, 6129.58, 6144.15, 6117.52, 6013.13, 5983.25, 5955.25,
        5956.06, 5861.57, 5954.50, 5849.72, 5778.15, 5842.63, 5738.52, 5770.20,
        5614.56, 5572.07, 5599.30, 5521.52, 5638.94
    ];
    let rsi = bulk_rsi(&data, SmoothedMovingAverage, 14);
    println!("Bulk RSIs: {:?}", rsi);

    // new price comes in
    data.push(5769.21);
    let single_rsi = single_rsi(&data[date.len() - 14..], SmoothedMovingAverage);
    println!("Single RSI: {}", single_rsi);
}
```

---

## 🧪 Output

> the code from this guide can be found in [`./examples/bulk_vs_single.rs`](./examples/bulk_vs_single.rs) and can be run in the terminal `cargo run --example bulk_vs_single`

```shell
Bulk RSIs: [47.49434607156126, 50.3221945432267, ..., 40.34609500741716]
Single RSI: 48.00106962275036
```

- Use bulk when: calculating many historical values, initial setup, backtesting
- Use single when: real-time updates, streaming data, memory constraints
