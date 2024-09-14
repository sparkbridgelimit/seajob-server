use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};

pub fn init_prom (prefix: &str) -> PrometheusMetrics{
    PrometheusMetricsBuilder::new(prefix)
        .endpoint("/metrics") // 暴露指标的路由
        .build()
        .unwrap()
}