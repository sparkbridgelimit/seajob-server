use serde::Deserialize;

#[derive(Deserialize)]
pub struct JobTaskList {
    pub job_define_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct JobTaskStart {
    pub job_task_id: i64,
}

#[derive(Deserialize)]
pub struct JobTaskLog {
    pub job_task_id: i64,
    pub job_name: Option<String>,
    pub job_link: Option<String>,
    pub company: Option<String>,
    pub boss_name: Option<String>,
    pub address: Option<String>,
    pub salary_range: Option<[i8; 2]>
}

#[derive(Deserialize)]
pub struct JobTaskError {
    pub job_task_id: i64,
    pub error: Option<String>
}

#[derive(Deserialize)]
pub struct JobTaskEnd {
    pub job_task_id: i64
}
