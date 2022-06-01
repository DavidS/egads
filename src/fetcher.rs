use reqwest::Client;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};

pub(crate) fn build_fetcher() -> ClientWithMiddleware {
    let reqwest_client = Client::new();
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

    ClientBuilder::new(reqwest_client)
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}
