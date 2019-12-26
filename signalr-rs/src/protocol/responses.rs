pub(crate) struct TransportDefinition {
    transport_name: String,
    transport_format: Vec<String>
}

pub(crate) struct NegotiateRequest {
    connection_token: String,
    connection_id: String,
    negotiate_version: u16,
    available_transports: Vec<TransportDefinition>
}