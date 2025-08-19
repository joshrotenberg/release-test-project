use release_test_core::DataModel;

pub fn format_data(model: &DataModel) -> String {
    format!("Data #{}: {} = {}", model.id, model.name, model.value)
}

pub fn serialize_data(model: &DataModel) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(model)
}

pub fn calculate_average(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

pub fn calculate_median(values: &mut [f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = values.len() / 2;
    let median = if values.len() % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[mid]
    };
    Some(median)
}

pub fn calculate_std_deviation(values: &[f64]) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    
    let mean = calculate_average(values);
    let variance = values.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / values.len() as f64;
    
    Some(variance.sqrt())
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
    }

    #[test]
    fn test_calculate_median() {
        let mut values = vec![1.0, 2.0, 3.0];
        assert_eq!(calculate_median(&mut values), Some(2.0));
        
        let mut values = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(calculate_median(&mut values), Some(2.5));
        
        let mut empty: Vec<f64> = vec![];
        assert_eq!(calculate_median(&mut empty), None);
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
}