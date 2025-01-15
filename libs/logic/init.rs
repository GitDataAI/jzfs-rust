use std::error::Error;

use tracing::{Subscriber, subscriber};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_log::LogTracer;
use tracing_subscriber::EnvFilter;

use crate::logic::subscriber::create_subscriber;

pub fn init_subscriber<S>(subscriber: S) -> Result<(), Box<dyn Error>>
where
    S: Subscriber + Send + Sync + 'static,
{
    LogTracer::init()?;
    subscriber::set_global_default(subscriber)?;
    Ok(())
}

pub fn init_tracing() -> Result<WorkerGuard, Box<dyn Error>> {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "logs", "app.log");
    let (file_appender, file_appender_guard) = tracing_appender::non_blocking(file_appender);
    init_subscriber(create_subscriber(
        "app",
        EnvFilter::from_default_env(),
        file_appender,
    ))?;
    Ok(file_appender_guard)
}
