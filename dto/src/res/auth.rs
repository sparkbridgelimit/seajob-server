use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SignInResponse {
    pub token: String,
    pub exp_at: usize
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SignUpResponse {
    pub token: String,
    pub exp_at: usize
}