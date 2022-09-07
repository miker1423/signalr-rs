
use std::{collections::HashMap, time::Duration};
use reqwest::Client;

pub mod protocol;
pub mod error;

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

    pub async fn build(&self) -> Option<protocol::responses::NegotiateRequest>  {
        if self.hub_url.is_empty() {
            return None;
        }
        let builder = Client::builder().build();
        
        let client = builder.unwrap();
        protocol::start_negotiation(&client, &self.hub_url).await
    }
}