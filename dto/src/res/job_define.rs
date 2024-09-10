use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct JobDefineListResponse {
    pub job_define_id: i64,
    pub job_define_name: String,
    pub job_define_desc: String,
    pub keyword: String,
    pub city_code: String,
    pub salary_range: String,
    pub total_apply: i32,
    pub last_run_time: DateTime<Utc>,
    pub create_time: DateTime<Utc>,
}

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
    pub wt2_cookie: String,
    pub hello_text: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct JobDefineRunResponse {
    // job_define
    pub job_task_id: i64,
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
    pub wt2_cookie: String,
    pub hello_text: String,
    // 运行次数
    pub target_num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobDefineCookieResponse {
    pub wt2_cookie: String,
}