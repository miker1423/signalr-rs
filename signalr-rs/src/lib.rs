
use std::collections::HashMap;

pub mod protocol;

#[derive(Default)]
pub struct HubConnectionBuilder {
    hub_url: String,
}

pub struct BaseHubConnectionBuilder;
pub struct HubConnection {
    listeners: HashMap<String, Box<dyn Executable>>
}

trait Executable {}

impl HubConnectionBuilder {
    pub fn new() -> HubConnectionBuilder {
        HubConnectionBuilder {
            ..Default::default()
        }
    }

    pub fn with_url(self, hub_url: String) -> HubConnectionBuilder {
        HubConnectionBuilder { hub_url }
    }

    pub fn build(&self) -> Option<protocol::responses::NegotiateRequest>  {
        if self.hub_url.is_empty() {
            return None;
        }
        let client = reqwest::Client::new();
        protocol::start_negotiation(&client, &self.hub_url)
    }
}