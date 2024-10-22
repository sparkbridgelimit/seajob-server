use serde::Deserialize;
use validator_derive::Validate;

#[derive(Deserialize)]
pub struct JobDefineCreateRequest {
    pub job_define_name: Option<String>,
    pub job_define_desc: Option<String>,
    pub keyword: Option<String>,
    pub city_code: Option<String>,
    pub salary_range: Option<[i8; 2]>,
    pub key_kills: Option<Vec<String>>,
    pub hello_text: Option<String>,
    pub exclude_company: Option<Vec<String>>,
    pub exclude_job: Option<Vec<String>>,
    pub filter_offline: Option<bool>,
}

#[derive(Deserialize)]
pub struct JobDefineUpdateRequest {
    pub id: i64,
    pub job_define_name: Option<String>,
    pub job_define_desc: Option<String>,
    pub keyword: Option<String>,
    pub city_code: Option<String>,
    pub salary_range: Option<[i8; 2]>,
    pub key_kills: Option<Vec<String>>,
    pub exclude_company: Option<Vec<String>>,
    pub exclude_job: Option<Vec<String>>,
    pub filter_offline: Option<bool>,
    pub hello_text: Option<String>,
    pub wt2_cookie: Option<String>,
}

#[derive(Deserialize)]
pub struct JobDefineRunRequest {
    pub job_define_id: i64,
    pub target_num: i32,
}

#[derive(Debug, Validate, Deserialize)]
pub struct JobDefineUserAllRequest {
    #[validate(range(min = 1, message = "user_id must be greater than 0"))]
    pub user_id: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct JobDefineDelete {
    #[validate(range(min = 1, message = "job_define_id must be greater than 0"))]
    pub job_define_id: i64,
}

#[derive(Debug, Validate, Deserialize)]
pub struct JobDefineDetailRequest {
    #[validate(range(min = 1, message = "job_define_id must be greater than 0"))]
    pub job_define_id: i64,
}

#[derive(Deserialize)]
pub struct JobDefineSaveCookieRequest {
    pub job_define_id: i64,
    pub cookie: Option<String>
}

#[derive(Deserialize)]
pub struct JobDefineCookieRequest {
    pub job_define_id: i64
}