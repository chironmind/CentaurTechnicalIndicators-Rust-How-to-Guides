# How to use McGinly Dynamic RustTI function

This guide shows how to use the mcGinley dyanmic bands, the logic here can be applied to other 
McGinley dyanmic functions.

---

## 🎯 Goal

- Use the McGinley Dynamic bands

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

### 2. Calculate the McGinley dynamic bands

The McGinley Dynamic uses a previous calculate value, if no previous McGinley dynamic is available we use 0.0

```rust

use rust_ti::candle_indicators::bulk::mcginley_dynamic_bands as bulk_md_bands;
use rust_ti::candle_indicators::single::mcginley_dynamic_bands as single_md_bands;
use rust_ti::DeviationModel::StandardDeviation;

[...]

let bands = bulk_md_bands(
    &prices, 
    StandardDeviation,
    2.0, 
    0.0,
    5
);

println!("Length of bands {}", bands.len());

[...]

```

### 3. Use last value to calculate next McGinley band

From step 2 we now have a previous McGinley dynamic which we will use in the `single` function

```rust

[...]

// Next price comes in
prices.push(5689.24);

let next_band = single_md_bands(
    &prices[247..],
    StandardDeviation,
    2.0,
    bands.last().unwrap().1
);

println!(
    "Lower band {}, McGinley dynamic {}, upper band {}", 
    next_band.0, next_band.1, next_band.2
);

[...]

```

---

## 🧪 Output

> to run the repo example `cargo run --example mcginley_dynamic < data.csv`

```shell
Loaded 251 prices
Length of bands 247
Lower band 5551.313227907162, McGinley dynamic 5665.614575300795, upper band 5779.9159226944275
```

---

## ✅ Next Steps

- Programatically choose a period
- Programatically choose a `Deviationodel`
- Programatically choose a deviation multiplier
- Combine all selections
- Introduce the notion of punishment to the rating system

