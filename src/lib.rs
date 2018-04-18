use std::cmp::Ordering;


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

    let inv_size = (size as f64).recip();

    (-inv_size * (0.5 * alpha).ln()).sqrt()
}


pub fn test<F>(sample: &[f64], alpha: f64, cdf: F) -> Output
where
    F: Fn(f64) -> f64,
{
    let inv_len = (sample.len() as f64).recip();

    let score = sample.iter()
        .cloned()
        .enumerate()
        .map(|(i, r2)| {
            let cdf = cdf(r2);
            let lower_ecdf = inv_len *  i      as f64;
            let upper_ecdf = inv_len * (i + 1) as f64;

            (cdf - lower_ecdf).max(upper_ecdf - cdf)
        })
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .unwrap();

    Output {
        critical_value: critical_value(alpha, sample.len()),
        score,
    }
}
