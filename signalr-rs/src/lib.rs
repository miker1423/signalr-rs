
use std::collections::HashMap;

mod protocol;

struct HubConnectionBuilder {
    hub_url: String,
}

struct BaseHubConnectionBuilder;
struct HubConnection {
    listeners: HashMap<String, dyn Executable>
}

struct Subscription<T> {
    name: String,
    function: dyn Fn<T>
}

trait Executable {}

impl BaseHubConnectionBuilder {
    fn new() -> HubConnectionBuilderNoUrl {
        HubConnectionBuilder
    }
}

impl HubConnectionBuilder {
    fn with_url(hub_url: String) -> HubConnectionBuilder {
        HubConnectionBuilder { hub_url }
    }

    fn build(&self)  {
        if self.hub_url.is_empty() {
            unimplemented!();
        }
        let client = reqwest::Client::new();
        if let Some(negotiation) = protocol::start_negotiation(&client, &self.hub_url) {

        } else {
            unimplemented!();
        }
    }
}