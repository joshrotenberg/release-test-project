use release_test_core::DataModel;

pub fn format_data(model: &DataModel) -> String {
    format!("Data #{}: {} = {}", model.id, model.name, model.value)
}

pub fn serialize_data(model: &DataModel) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(model)
}

pub fn calculate_average(values: &[f64]) -> f64 {
    let valid_values: Vec<f64> = values.iter().copied().filter(|x| x.is_finite()).collect();

    if valid_values.is_empty() {
        return 0.0;
    }
    valid_values.iter().sum::<f64>() / valid_values.len() as f64
}

pub fn calculate_median(values: &mut [f64]) -> Option<f64> {
    // Filter out NaN and infinite values for consistency with other statistics functions
    let mut valid_values: Vec<f64> = values.iter().copied().filter(|x| x.is_finite()).collect();

    if valid_values.is_empty() {
        return None;
    }

    valid_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mid = valid_values.len() / 2;
    let median = if valid_values.len() % 2 == 0 {
        (valid_values[mid - 1] + valid_values[mid]) / 2.0
    } else {
        valid_values[mid]
    };
    Some(median)
}

pub fn calculate_variance(values: &[f64]) -> Option<f64> {
    let valid_values: Vec<f64> = values.iter().copied().filter(|x| x.is_finite()).collect();

    if valid_values.is_empty() {
        return None;
    }

    let mean = calculate_average(&valid_values);
    let variance =
        valid_values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / valid_values.len() as f64;

    Some(variance)
}

pub fn calculate_std_deviation(values: &[f64]) -> Option<f64> {
    calculate_variance(values).map(|v| v.sqrt())
}

pub fn find_min(values: &[f64]) -> Option<f64> {
    values
        .iter()
        .copied()
        .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
}

pub fn find_max(values: &[f64]) -> Option<f64> {
    values
        .iter()
        .copied()
        .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
}

#[cfg(test)]
mod tests {
    use super::*;
    use release_test_core::DataModel;

    #[test]
    fn test_format_data() {
        let model = DataModel::new(1, "test".to_string(), 42.0).unwrap();
        let formatted = format_data(&model);
        assert_eq!(formatted, "Data #1: test = 42");
    }

    #[test]
    fn test_serialize_data() {
        let model = DataModel::new(1, "test".to_string(), 42.0).unwrap();
        let json = serialize_data(&model).unwrap();
        assert!(json.contains("\"id\": 1"));
        assert!(json.contains("\"name\": \"test\""));
    }

    #[test]
    fn test_calculate_average() {
        assert_eq!(calculate_average(&[1.0, 2.0, 3.0]), 2.0);
        assert_eq!(calculate_average(&[]), 0.0);

        // Test NaN handling
        assert_eq!(calculate_average(&[1.0, 2.0, f64::NAN, 3.0]), 2.0);
        assert_eq!(calculate_average(&[f64::NAN, f64::NAN]), 0.0);
        assert_eq!(calculate_average(&[f64::INFINITY, 1.0, 2.0]), 1.5);
    }

    #[test]
    fn test_calculate_median() {
        let mut values = vec![1.0, 2.0, 3.0];
        assert_eq!(calculate_median(&mut values), Some(2.0));

        let mut values = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(calculate_median(&mut values), Some(2.5));

        let mut empty: Vec<f64> = vec![];
        assert_eq!(calculate_median(&mut empty), None);

        // Test NaN handling
        let mut values = vec![1.0, 2.0, f64::NAN, 3.0, 4.0];
        assert_eq!(calculate_median(&mut values), Some(2.5));

        // Test with all NaN values
        let mut values = vec![f64::NAN, f64::NAN];
        assert_eq!(calculate_median(&mut values), None);

        // Test with infinity
        let mut values = vec![1.0, 2.0, f64::INFINITY, 3.0];
        assert_eq!(calculate_median(&mut values), Some(2.0));
    }

    #[test]
    fn test_calculate_variance() {
        // Test with simple values
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let variance = calculate_variance(&values).unwrap();
        assert!((variance - 4.0).abs() < 0.01);

        // Test with single value
        let values = vec![5.0];
        assert_eq!(calculate_variance(&values), Some(0.0));

        // Test with empty array
        let empty: Vec<f64> = vec![];
        assert_eq!(calculate_variance(&empty), None);

        // Test with identical values
        let values = vec![3.0, 3.0, 3.0, 3.0];
        assert_eq!(calculate_variance(&values), Some(0.0));

        // Test NaN handling
        let values = vec![2.0, 4.0, f64::NAN, 6.0];
        let variance = calculate_variance(&values).unwrap();
        assert!((variance - 2.67).abs() < 0.01); // Variance of [2,4,6]
    }

    #[test]
    fn test_calculate_std_deviation() {
        // Test with simple values
        let values = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std_dev = calculate_std_deviation(&values).unwrap();
        assert!((std_dev - 2.0).abs() < 0.01);

        // Test with single value
        let values = vec![5.0];
        assert_eq!(calculate_std_deviation(&values), Some(0.0));

        // Test with empty array
        let empty: Vec<f64> = vec![];
        assert_eq!(calculate_std_deviation(&empty), None);

        // Test with identical values
        let values = vec![3.0, 3.0, 3.0, 3.0];
        assert_eq!(calculate_std_deviation(&values), Some(0.0));
    }

    #[test]
    fn test_find_min() {
        assert_eq!(find_min(&[3.0, 1.0, 4.0, 2.0]), Some(1.0));
        assert_eq!(find_min(&[42.0]), Some(42.0));
        assert_eq!(find_min(&[]), None);
        assert_eq!(find_min(&[-5.0, 0.0, 5.0]), Some(-5.0));
    }

    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[3.0, 1.0, 4.0, 2.0]), Some(4.0));
        assert_eq!(find_max(&[42.0]), Some(42.0));
        assert_eq!(find_max(&[]), None);
        assert_eq!(find_max(&[-5.0, 0.0, 5.0]), Some(5.0));
    }
}
