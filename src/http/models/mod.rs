use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Clone, Deserialize, Debug, Validate, Default)]
pub struct ValidationRequest {
    #[validate(required, length(min = 1))]
    pub message: Option<String>,
}

#[derive(Clone, Serialize, Debug, Default)]
pub struct ValidationResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthCheckResponse {
    pub message: String,
}

impl Default for HealthCheckResponse {
    fn default() -> Self {
        Self {
            message: "Ok".to_string(),
        }
    }
}
