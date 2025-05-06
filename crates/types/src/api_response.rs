use std::fmt::Debug;
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::{ValidationError, ValidationErrors};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct SuccessApiResponse<D> {
    pub data: D,
    pub status: String,
}

impl <D> SuccessApiResponse<D>
where D: Debug + Clone {
    pub fn new(data: &D) -> Self {
        Self {
            data: data.clone(),
            status: "success".to_string()
        }
    }
}

impl <D> Into<serde_json::Value>
for SuccessApiResponse<D>
where D: Serialize {
    fn into(self) -> serde_json::Value {
        json!({
            "status": self.status,
            "data": json!(self.data),
        })
    }
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(PartialEq, PartialOrd)]
pub struct ErrorApiResponseData {
    pub code: String,
    pub message: Option<String>,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorApiResponse {
    pub status: String,
    pub errors: Vec<ErrorApiResponseData>,
}

impl ErrorApiResponse {
    pub fn new(errors: Vec<ErrorApiResponseData>) -> Self {
        Self {
            errors: errors.clone(),
            status: "error".to_string(),
        }
    }
}

impl Into<serde_json::Value>
for ErrorApiResponse {
    fn into(self) -> serde_json::Value {
        json!({
            "status": self.status,
            "errors": self.errors,
        })
    }
}

impl ErrorApiResponseData {
    pub fn new(code: &str, message: &Option<String>) -> Self {
        Self {
            code: code.to_string(),
            message: message.clone().map(|message| message.to_string()),
            timestamp: Utc::now()
                .naive_local()
                .into()
        }
    }
}

impl From<ValidationError> for ErrorApiResponseData {
    fn from(value: ValidationError) -> Self {
        Self::new(
            &value.code,
            &value.message.map(|message| message.to_string())
        )
    }
}

impl From<ValidationErrors> for ErrorApiResponse {
    fn from(value: ValidationErrors) -> Self {
        let errors = value
            .field_errors()
            .iter()
            .map(|(_field_name, error)| error
                .iter()
                .map(|error| ErrorApiResponseData::from(error.clone()))
            )
            .flatten()
            .collect();

        Self::new(errors)
    }
}

impl From<ErrorApiResponseData> for ErrorApiResponse {
    fn from(value: ErrorApiResponseData) -> Self {
        Self::new(vec![value])
    }
}
