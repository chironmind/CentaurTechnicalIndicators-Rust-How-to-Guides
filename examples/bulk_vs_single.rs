use rust_ti::momentum_indicators::bulk::relative_strength_index as bulk_rsi;
use rust_ti::momentum_indicators::single::relative_strength_index as single_rsi;
use rust_ti::ConstantModelType::SmoothedMovingAverage;

fn main() {
    let mut data = vec![
        6037.59, 5970.84, 5906.94, 5881.63, 5868.55, 5942.47, 5975.38, 5909.03, 5918.25, 5827.04,
        5836.22, 5842.91, 5949.91, 5937.34, 5996.66, 6049.24, 6086.37, 6118.71, 6101.24, 6012.28,
        6067.70, 6039.31, 6071.17, 6040.53, 5994.57, 6037.88, 6061.48, 6083.57, 6025.99, 6066.44,
        6068.50, 6051.97, 6115.07, 6114.63, 6129.58, 6144.15, 6117.52, 6013.13, 5983.25, 5955.25,
        5956.06, 5861.57, 5954.50, 5849.72, 5778.15, 5842.63, 5738.52, 5770.20, 5614.56, 5572.07,
        5599.30, 5521.52, 5638.94,
    ];
    let rsi = bulk_rsi(&data, SmoothedMovingAverage, 14);
    println!("Bulk RSIs: {:?}", rsi);

    // new price comes in
    data.push(5769.21);
    let single_rsi = single_rsi(&data[40..], SmoothedMovingAverage);
    println!("Single RSI: {}", single_rsi);
}
