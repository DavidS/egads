// mod descriptor;
// mod list;

use clap::Parser;
use reqwest::Client;
// use std::{fs::File, io::BufReader};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[tokio::main]
async fn main() {
    // println!("Hello, world!");
    // let args = Cli::parse();
    // println!("{:#?}", args.path);

    // let file = File::open(&args.path).expect("could not open file");
    // let reader = BufReader::new(file);
    // let api: definitions::ApiDescriptor =
    //     serde_json::from_reader(reader).expect("couldn't parse service description");

    // // println!("{:#?}", api.resources["subscriptions"].methods["list"]);
    // println!("{:#?}", api.schemas["SubscriptionListResponse"]);

    let client = Client::new();
    let list = egads::list::fetch(&client)
        .await
        .expect("Failed to load directory list");
    // println!("{:#?}", list);

    for item in list.items {
        println!("Fetching {} from {}", item.title, item.discovery_rest_url);
        let descriptor = egads::descriptor::fetch_item(&client, &item)
            .await
            .expect("error retrieving item descriptor");
        println!("{:#?}", descriptor);
    }
}
