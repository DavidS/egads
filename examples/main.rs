use egads::descriptor::from_str;
use egads::fetcher::build_fetcher;
use futures::stream;
use futures::StreamExt;
use opentelemetry::global::shutdown_tracer_provider;
use tracing::{debug, info, info_span, Instrument};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        // .install_simple()?;
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
    //.with(fmt::layer());
    // .with(EnvFilter::from_default_env());
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    exercise_egads().await;

    shutdown_tracer_provider();
    Ok(())
}


async fn exercise_egads() {
    let span = info_span!(target: "egads-test", "exercise_egads");
    let _enter = span.enter();

    let client = build_fetcher();

    let list = egads::list::fetch(&client)
        .await
        .expect("Failed to load directory list");
    debug!(?list);

    let mut successes = 0;
    let mut errors = 0;

    stream::iter(list.items.into_iter())
        .map(|item| {
            let client = &client;
            let url = item.discovery_rest_url;
            async move {
                debug!("Starting descriptor fetch");
                let resp = client.get(url).send().await?;
                debug!("Fetching descriptor response");
                let body = resp.text().await?;
                debug!("Parsing response");
                from_str(body)
            }
            .instrument(info_span!("fetch and process"))
        })
        .buffer_unordered(150)
        .map(|r| match r {
            Ok(_) => successes += 1,
            Err(_) => errors += 1,
        })
        .collect::<Vec<_>>()
        .await;

    info!("processed: {} successes, {} errors", successes, errors);
}
