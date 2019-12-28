
pub mod responses;

use responses::NegotiateRequest;
use reqwest;

pub(crate) fn start_negotiation(client: &reqwest::Client, url: &str) -> Option<NegotiateRequest> {
    let url = format!("{}/negotiate?negotiateVersion=1", url);
    let mut result = client.post(&url).header("Content-Length", "0").send().unwrap();
    if result.status().is_success() {
        result.json().unwrap()
    } else {
        None
    }
}