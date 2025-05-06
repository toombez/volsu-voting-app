use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde_json::json;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorApiResponse<E> {
    pub status: String,
    pub errors: Vec<E>,
}

impl <E> ErrorApiResponse<E>
where E: Debug + Clone {
    pub fn new(errors: &Vec<E>) -> Self {
        Self {
            errors: errors.clone(),
            status: "error".to_string(),
        }
    }
}

impl <E> Into<serde_json::Value>
for ErrorApiResponse<E>
where E: Serialize {
    fn into(self) -> serde_json::Value {
        json!({
            "status": self.status,
            "errors": json!(self.errors),
        })
    }
}
