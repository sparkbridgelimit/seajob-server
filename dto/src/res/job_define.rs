use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JobDefineDetailResponse {
    // job_define
    pub job_define_id: i64,
    pub job_define_name: String,
    pub job_define_desc: String,
    // job_prefer
    pub keyword: String,
    pub city_code: String,
    pub salary_range: String,
    pub key_kills: String,
    pub exclude_company: String,
    pub exclude_job: String,
    // job_param
    pub interval: i32,
    pub timeout: i32,
    pub greet_num: i32,
    pub wt2_cookie: String
}
