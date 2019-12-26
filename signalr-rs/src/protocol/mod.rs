
mod responses;

use responses::NegotiateRequest;
use reqwest;

pub(crate) fn start_negotiation(client: &reqwest::Client, url: &str) -> Option<NegotiateRequest> {
    let mut result = client.post(url).send().unwrap();
    if result.status().is_success() {
        result.json().unwrap()
    } else {
        None
    }
}