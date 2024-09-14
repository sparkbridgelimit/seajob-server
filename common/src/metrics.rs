use actix_web_prom::{PrometheusMetrics, PrometheusMetricsBuilder};

pub fn init_prom () -> PrometheusMetrics{
    PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics") // 暴露指标的路由
        .build()
        .unwrap()
}