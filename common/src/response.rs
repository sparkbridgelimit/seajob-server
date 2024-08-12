use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    err_code: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    err_message: Option<String>,
}

pub type ApiErrorResponse = ApiResponse<()>;

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            err_code: None,
            err_message: None,
        }
    }
}
impl ApiResponse<()> {
    pub fn success_only() -> Self {
        ApiResponse {
            success: false,
            data: None,
            err_code: None,
            err_message: None,
        }
    }
}

// 专门为 ApiResponse<()> 定义 fail 和 fail_with_error 方法
impl ApiResponse<()> {
    pub fn fail() -> Self {
        ApiResponse {
            success: false,
            data: None,
            err_code: Some(0),
            err_message: Some("系统异常".to_string()),
        }
    }

    pub fn fail_with_error(err: ApiErr) -> Self {
        ApiResponse {
            success: false,
            data: None,
            err_code: Some(err.code()),
            err_message: Some(err.message().to_string()),
        }
    }
}

pub enum ApiErr {
    SYSTEM,
    NotFound,
    UNAUTHORIZED,
    ValidationErrors,
    NotAuth,
}

impl ApiErr {
    fn code(&self) -> u32 {
        match self {
            ApiErr::SYSTEM => 1000,
            ApiErr::NotFound => 1001,
            ApiErr::UNAUTHORIZED => 1002,
            ApiErr::ValidationErrors => 1003,
            ApiErr::NotAuth => 1004
        }
    }

    fn message(&self) -> &str {
        match self {
            ApiErr::SYSTEM => "系统异常",
            ApiErr::NotFound => "未找到资源",
            ApiErr::UNAUTHORIZED => "未授权",
            ApiErr::ValidationErrors => "参数校验未通过",
            ApiErr::NotAuth => "未认证"
        }
    }
}
