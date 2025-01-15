use std::time::Duration;

use opentelemetry::KeyValue;
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::fmt::{self, MakeWriter};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

pub(crate) fn create_subscriber<W>(
    name: &str,
    env_filter: EnvFilter,
    writer: W,
) -> impl Subscriber + Sync + Send
where
    W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://0.0.0.0:4317")
        .with_timeout(Duration::from_secs(3))
        .build()
        .expect("OTLP exporter failed");
    // 追踪导出器
    let tracer_provider = opentelemetry_sdk::trace::TracerProvider::builder()
        .with_batch_exporter(exporter, opentelemetry_sdk::runtime::Tokio)
        .with_resource(Resource::new(vec![KeyValue::new(
            "service.name",
            name.to_string(),
        )]))
        .build();
    // 创建追踪器对象
    let tracer = tracer_provider.tracer(name.to_string());
    // 创建格式化层
    let fmt_layer = fmt::Layer::default().with_file(false).with_target(false);
    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(OpenTelemetryLayer::new(tracer))
        .with(JsonStorageLayer)
        .with(BunyanFormattingLayer::new(name.into(), std::io::stdout))
        .with(BunyanFormattingLayer::new(name.into(), writer))
}
