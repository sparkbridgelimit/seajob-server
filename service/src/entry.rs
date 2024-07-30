use crate::job_define::JobDefineService;
use std::sync::OnceLock;

pub static JOB_DEFINE_SERVICE: OnceLock<JobDefineService> = OnceLock::new();

pub fn init_services() {
    JOB_DEFINE_SERVICE.get_or_init(|| JobDefineService::new());
}
