
pub mod responses;

use responses::NegotiateRequest;
use reqwest;

pub(crate) async fn start_negotiation(client: &reqwest::Client, url: &str) -> Option<NegotiateRequest> {
    let url = format!("{}/negotiate?negotiateVersion=1", url);
    let result = client.post(&url)
                                                .header("Content-Length", "0")
                                                .send()
                                                .await;
    if let Err(_) = result { return None; }
    let result = result.unwrap(); 
    if result.status().is_success() {
        let response = result.json::<NegotiateRequest>().await;
        Some(response.unwrap())
    } else {
        None
    }
}