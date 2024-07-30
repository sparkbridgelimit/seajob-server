use serde::Deserialize;
use validator_derive::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct JobContactedLog {
    #[validate(range(min = 1, message = "job_task_id must be greater than 0"))]
    pub job_task_id: i64,
    #[validate(length(min = 1, message = "job_name cannot be empty"))]
    pub job_name: Option<String>,
    pub job_link: Option<String>,
    pub company: Option<String>,
    pub boss_name: Option<String>,
    pub address: Option<String>,
    pub salary_range: Option<[i8; 2]>,
}

#[derive(Debug, Validate, Deserialize)]
pub struct JobContactedUser {
    #[validate(range(min = 1, message = "user_id must be greater than 0"))]
    pub user_id: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct JobContactedDefine {
    #[validate(range(min = 1, message = "job_define_id must be greater than 0"))]
    pub job_define_id: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct JobContactedTaskReq {
    #[validate(range(min = 1, message = "job_define_id must be greater than 0"))]
    pub job_task_id: i64,
}
