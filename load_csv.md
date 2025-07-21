# How to Load CSV Price Data into RustTI

This guide shows how to read a CSV file with historical OHLC price data, store the values into a struct, and
 extract those in a Vec suitable for use with RustTI indicators.

---

## 🎯 Goal

- Load a `.csv` file with columns like timestamp, open, high, low, close
- Parse the coumns into an OHLC struct
- Store the enums into a Vec

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

### 1. Prepare your CSV file

Example format:

```csv
date,open,high,low,close
03/14/2024,5175.14,5176.85,5123.30,5150.48
03/15/2024,5123.31,5136.86,5104.35,5117.09
03/18/2024,5154.77,5175.60,5145.47,5149.42
03/19/2024,5139.09,5180.31,5131.59,5178.51
03/20/2024,5181.69,5226.19,5171.55,5224.62
```

Save it as data.csv

### 2. Define a struct to map each row

```rust

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Ohlc {
    #[serde(with = "csv_date_format")]
    date: NaiveDate,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

```

### 3. Create a `mod` to handle the date

```rust

use chrono::NaiveDate;

[...]

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

```

### 4. Read and parse the file

```rust

use std::io;

[...]

fn get_data() -> Vec<Ohlc> {
    let mut prices = Vec::new();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for line in rdr.deserialize() {
        let ohlc: Ohlc = line.expect("");
        prices.push(ohlc);
    }   
    return prices;
}

```

### 5. Use it in your main function

```rust

fn main() {
    let data = get_data();
    println!("Loaded {} prices", data.len());
}

```

### 6. Calculate the RSI

You will want to choose a `period`, and `ConstantModelType`. 
[Choosing a period](./choose_period.md) and [Choosing a constant model type](./choose_constant_model_type.md) 
show you how to do this in more programatically, for now we will use the defaults that Welles used when creating the RSI.

```rust

use rust_ti::momentum_indicators::bulk::relative_strength_index;
use rust_ti::ConstantModelType::SmoothedMovingAverage;

[...]

fn main() {
    let data = get_data();
    println!("Loaded {} prices", data.len());

    let close: Vec<f64> = data.iter().map(|i| i.close).collect();
    let rsi = relative_strength_index(&close, SmoothedMovingAverage, 14);
    println!("Calculated {} rsis", rsi.len());
}

```

### 7. Run the code

```bash
cargo run < data.csv
```

or to run this how-to example

```bash
cargo run --example load_csv < data.csv
```

---

## 🧪 Output

```shell
Loaded 251 prices
Calculated 238 rsis
```

---

## ✅ Next Steps

- [Programatically choose a period](./choose_period.md) 
- [Programatically choose a constant type model](./choose_constant_model_type.md) 
