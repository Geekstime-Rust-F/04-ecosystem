use std::{
    io,
    time::{Duration, Instant},
};

use anyhow::{Ok, Result};
use axum::{extract::Request, routing::get, Router};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    trace::{self, RandomIdGenerator, Tracer},
    Resource,
};
use tokio::{net::TcpListener, time::sleep};
use tracing::{debug, info, instrument, level_filters::LevelFilter, warn};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer, Registry,
};

#[tokio::main]
async fn main() -> Result<()> {
    // tail -f /tmp/logs/ecosystem.log.2024-06-22-16
    // file layer
    let file_appender = tracing_appender::rolling::hourly("/tmp/logs", "ecosystem.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .with_filter(LevelFilter::WARN);

    // opentelemetry layer
    let tracer = init_tracer()?;
    let tracer_layer = OpenTelemetryLayer::new(tracer).with_filter(LevelFilter::INFO);

    // console layer
    let console_layer = fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_level(true)
        .pretty()
        .with_writer(io::stdout)
        .with_filter(LevelFilter::INFO);

    Registry::default()
        .with(console_layer)
        .with(file_layer)
        .with(tracer_layer)
        .init();
    info!("init tracing");

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("App listne to: {:?}", addr); // 打启动日志的好处之一就是通过这个可以清晰地看到程序是否重启了
    let app = Router::new().route("/", get(index_handler));

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument(fields(http.uri = req.uri().path(), http.method = req.method().as_str()))]
async fn index_handler(req: Request) -> &'static str {
    debug!("index_handler starting");
    sleep(Duration::from_millis(12)).await;
    long_task().await;
    info!(http.status_code = 200, "index handler completed");
    "hello"
}

#[instrument]
async fn long_task() -> &'static str {
    let start = Instant::now();
    sleep(Duration::from_millis(122)).await;
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.task_duration = elapsed, "task takes too long");

    "hello long task"
}

fn init_tracer() -> Result<Tracer> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint("http://localhost:4317"),
        )
        .with_trace_config(
            trace::config()
                .with_id_generator(RandomIdGenerator::default())
                .with_resource(Resource::new(vec![KeyValue::new(
                    "service.name",
                    "axum-tracing",
                )])),
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    Ok(tracer)
}
