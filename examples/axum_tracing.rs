use std::time::{Duration, Instant};

use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use tokio::{net::TcpListener, time::sleep};
use tracing::{debug, info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer, Registry,
};

#[tokio::main]
async fn main() -> Result<()> {
    // tail -f /tmp/logs/ecosystem.log.2024-06-22-16
    let file_appender = tracing_appender::rolling::hourly("/tmp/logs", "ecosystem.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(non_blocking)
        .with_filter(LevelFilter::WARN);

    let layer = fmt::layer()
        .with_span_events(FmtSpan::CLOSE)
        .with_level(true)
        .pretty()
        .with_filter(LevelFilter::DEBUG);
    Registry::default().with(layer).with(file_layer).init();
    info!("init tracing");

    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await?;
    info!("App listne to: {:?}", addr); // 打启动日志的好处之一就是通过这个可以清晰地看到程序是否重启了
    let app = Router::new().route("/", get(index_handler));

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[instrument]
async fn index_handler() -> &'static str {
    debug!("index_handler starting");
    long_task().await;
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
