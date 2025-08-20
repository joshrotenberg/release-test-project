use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("processing error: {0}")]
    ProcessingError(String),

    #[error("validation error: {0}")]
    ValidationError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataModel {
    pub id: u64,
    pub name: String,
    pub value: f64,
}

impl DataModel {
    pub fn new(id: u64, name: String, value: f64) -> Result<Self, CoreError> {
        if name.is_empty() {
            return Err(CoreError::ValidationError(
                "Name cannot be empty".to_string(),
            ));
        }
        Ok(Self { id, name, value })
    }

    pub fn process(&self) -> f64 {
        self.value * 2.0
    }

    pub fn squared(&self) -> f64 {
        self.value * self.value
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_model_creation() {
        let model = DataModel::new(1, "test".to_string(), 42.0).unwrap();
        assert_eq!(model.id, 1);
        assert_eq!(model.name, "test");
        assert_eq!(model.value, 42.0);
    }

    #[test]
    fn test_data_model_validation() {
        let result = DataModel::new(1, "".to_string(), 42.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_process() {
        let model = DataModel::new(1, "test".to_string(), 21.0).unwrap();
        assert_eq!(model.process(), 42.0);
    }

    #[test]
    fn test_to_json() {
        let model = DataModel::new(1, "test".to_string(), 42.0).unwrap();
        let json = model.to_json().unwrap();
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"name\":\"test\""));
    }
}
