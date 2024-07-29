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
    // #[validate(custom = "validate_salary_range")]
    pub salary_range: Option<[i8; 2]>,
}

// 自定义验证函数用于 salary_range
// fn validate_salary_range(salary_range: &[i8; 2]) -> Result<(), ValidationError> {
//     if salary_range[0] < 0 || salary_range[1] < 0 || salary_range[0] > salary_range[1] {
//         return Err(ValidationError::new("invalid salary range"));
//     }
//     Ok(())
// }