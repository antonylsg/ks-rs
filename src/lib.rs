#[macro_use]
extern crate lazy_static;

mod table;

use std::cmp::Ordering;

lazy_static! {
    static ref TABLE: table::Table = table::Table::new();
}

pub struct Output {
    pub critical_value: f64,
    pub score: f64,
}

impl Output {
    pub fn is_accepted(&self) -> bool {
        self.score <= self.critical_value
    }
}

fn critical_value(alpha: f64, size: usize) -> f64 {
    assert!(0.0 < alpha && alpha < 1.0);

    let alpha = (100.0 * alpha) as u8;

    TABLE.get((size, alpha)).unwrap()
}

pub fn test<F>(sample: &[f64], alpha: f64, cdf: F) -> Option<Output>
where
    F: Fn(f64) -> f64,
{
    let inv_len = (sample.len() as f64).recip();

    let score = sample
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, x)| {
            let cdf = cdf(x);
            let lower_ecdf = inv_len * idx as f64;
            let upper_ecdf = inv_len * (idx + 1) as f64;

            (cdf - lower_ecdf).max(upper_ecdf - cdf)
        }).max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))?;

    Some(Output {
        critical_value: critical_value(alpha, sample.len()),
        score,
    })
}
