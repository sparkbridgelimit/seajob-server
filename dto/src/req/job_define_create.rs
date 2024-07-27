use serde::Deserialize;

#[derive(Deserialize)]
pub struct JobDefineCreateRequest {
    pub job_define_name: Option<String>,
    pub job_define_desc: Option<String>,
    pub user_id: Option<i64>,
    pub greet_num: Option<i32>,
    pub keyword: Option<String>,
    pub city_code: Option<String>,
    pub salary_range: Option<[i8; 2]>,
    pub key_kills: Option<Vec<String>>,
    // ['蚂蚁集团']
    pub exclude_company: Option<Vec<String>>,
    // ['远程']
    pub exclude_job: Option<Vec<String>>,
}