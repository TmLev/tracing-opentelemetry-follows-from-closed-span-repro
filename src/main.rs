use opentelemetry_sdk::export::trace::stdout;

use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
async fn main() {
    setup_environment();

    // Submit some job that will complete asynchronously.
    let job_span_id = submit_async_job().await;

    // Wait some time before the job completes.
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    // Receive the result of the job.
    receive_job_result(job_span_id).await;
}

#[tracing::instrument]
async fn submit_async_job() -> Option<tracing::Id> {
    let job_span = tracing::info_span!(parent: None, "Async job").entered();

    // Actually send the job to some external service.
    // ...

    job_span.id()
}

#[tracing::instrument]
async fn receive_job_result(job_span_id: Option<tracing::Id>) {
    // This line panics, because `tracing_opentelemetry` can't find
    // already closed span in its OpenTelemetry context.
    tracing::Span::current().follows_from(job_span_id);
}

fn setup_environment() {
    // Env variables.

    std::env::set_var("RUST_LOG", "debug");

    // Tracing.

    let opentelemetry = {
        let tracer = stdout::new_pipeline().install_simple();
        tracing_opentelemetry::layer().with_tracer(tracer)
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .with(opentelemetry)
        .init();
}
