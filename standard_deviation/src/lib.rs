/// The population mean is the average of every measurement in the population: (Σ x) / n where n is the number of measurements in the population.
///
/// Assuming that we have every measurement in the population.
pub fn population_mean(measurements: &[i64]) -> i64 {
    assert!(!measurements.is_empty());
    measurements.iter().sum::<i64>() / measurements.len() as i64
}

/// The estimated population mean is the average of the measurements we have: (Σ x) / n where n is the number of measurements we have.
///
/// Assuming we do not have every measurement in the population.
pub fn estimate_population_mean(measurements: &[i64]) -> i64 {
    assert!(!measurements.is_empty());
    measurements.iter().sum::<i64>() / measurements.len() as i64
}

/// Population variance is Σ (x - μι)² / n
/// Population standard deviation is sqrt ((Σ (x - μι)² / n)
pub fn population_standard_deviation(measurements: &[i64]) -> i64 {
    let mean = population_mean(measurements);
    let variance =
        measurements.iter().map(|x| (x - mean).pow(2)).sum::<i64>() / measurements.len() as i64;
    (variance as f64).sqrt() as i64
}

/// Estimated population variance is Σ (x - x̂)² / n - 1
/// Estimated population standard deviation is sqrt (Σ (x - x̂)² / n - 1)
pub fn estimate_population_standard_deviation(measurements: &[i64]) -> i64 {
    let estimated_mean = estimate_population_mean(measurements);
    let estimated_variance = measurements
        .iter()
        .map(|x| (x - estimated_mean).pow(2))
        .sum::<i64>()
        / (measurements.len() - 1) as i64;
    (estimated_variance as f64).sqrt() as i64
}
