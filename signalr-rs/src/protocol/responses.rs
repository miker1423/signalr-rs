use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TransportDefinition {
    #[serde(rename = "transport")]
    transport_name: String,
    #[serde(rename = "transferFormats")]
    transport_format: Vec<String>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NegotiateRequest {
    #[serde(rename = "connectionToken")]
    connection_token: String,
    #[serde(rename = "connectionId")]
    connection_id: String,
    #[serde(rename = "negotiateVersion")]
    negotiate_version: u16,
    #[serde(rename = "availableTransports")]
    available_transports: Vec<TransportDefinition>
}