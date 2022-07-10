use clap::Parser;
use opentelemetry::global::shutdown_tracer_provider;
use tracing::{info, instrument, warn};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        // .install_simple()?;
        .install_batch(opentelemetry::runtime::Tokio)?;

    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry).with(fmt::layer());
    tracing::subscriber::set_global_default(subscriber).expect("setting tracing default failed");

    exercise_egads().await;

    shutdown_tracer_provider();
    Ok(())
}


async fn exercise_egads() {
    let span = info_span!(target: "egads-test", "exercise_egads");
    let _enter = span.enter();

    // println!("Hello, world!");
    // let args = Cli::parse();
    // println!("{:#?}", args.path);

    // let file = File::open(&args.path).expect("could not open file");
    // let reader = BufReader::new(file);
    // let api: definitions::ApiDescriptor =
    //     serde_json::from_reader(reader).expect("couldn't parse service description");

    // // println!("{:#?}", api.resources["subscriptions"].methods["list"]);
    // println!("{:#?}", api.schemas["SubscriptionListResponse"]);

    let list = egads::list::fetch()
        .await
        .expect("Failed to load directory list");
    // println!("{:#?}", list);

    let mut checks = Vec::new();
    for item in list.items {
        // clone the span for each fetch
        let span = span.clone();
        checks.push(tokio::spawn(async move {
            let _descriptor = egads::descriptor::fetch_item(&item)
                .instrument(span)
                .await
                .expect("error retrieving item descriptor");
        }));
    }

    let total = checks.len();
    let mut successes = 0;
    let mut errors = 0;

    for check in checks {
        match check.await.ok() {
            Some(_) => successes += 1,
            None => errors += 1,
        }
    }

    info!(
        "{} items processed: {} successes, {} errors",
        total, successes, errors
    );
    // let mut done = true;
    // let current = "set this to a api name";

    // for item in list.items {
    //     if current == item.name {
    //         done = false;
    //     }
    //     if done {
    //         continue;
    //     }
    //     println!(
    //         "Fetching {} ({}) from {}",
    //         item.name, item.title, item.discovery_rest_url
    //     );
    //     let descriptor = egads::descriptor::fetch_item(&client, &item)
    //         .await
    //         .expect("error retrieving item descriptor");
    //     println!("{:#?}", descriptor.description);
    // }
}
