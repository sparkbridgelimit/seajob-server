use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateActivateCodeReq {
    pub biz_code: String,
    pub duration: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumeActivateCodeReq {
    pub code: String,
}