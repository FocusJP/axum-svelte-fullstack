use std::time::Duration;

use axum::{
    extract::{MatchedPath, Request},
    response::Response,
};
use opentelemetry::trace::TracerProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
    Resource,
    trace::{RandomIdGenerator, Sampler, SdkTracerProvider},
};
use tracing::{Span, field::Empty, info_span};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::error::AppError;

pub fn init() -> Result<(), AppError> {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()?;

    let otel_endpoint = std::env::var("OTEL_ENDPOINT")?;

    let fmt_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_span_list(true)
        .with_current_span(false);

    let otel_exporter = opentelemetry_otlp::SpanExporter::builder()
        .with_tonic()
        .with_endpoint(otel_endpoint)
        .build()
        .unwrap();

    let otel_resource = Resource::builder()
        .with_service_name(env!("CARGO_PKG_NAME"))
        .build();

    let otel_tracer_provider = SdkTracerProvider::builder()
        .with_sampler(Sampler::TraceIdRatioBased(1.0))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(otel_resource)
        .with_batch_exporter(otel_exporter)
        .build()
        .tracer("tracing-otel-subscriber");

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(OpenTelemetryLayer::new(otel_tracer_provider))
        .init();

    Ok(())
}

pub fn trace_layer_make_span(request: &Request) -> Span {
    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(MatchedPath::as_str);

    let uri = request.uri().to_string();

    let method = request.method().to_string();

    info_span!(
        "http_request",
        "http.method" = method,
        "http.route" = matched_path,
        "http.uri" = uri,
        "http.status_code" = Empty,
    )
}

pub fn trace_layer_on_response(response: &Response, _latency: Duration, span: &Span) {
    span.record("http.status_code", response.status().as_u16());
}
