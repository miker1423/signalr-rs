use serde::{Deserialize, Serialize};
use serde_json::Value;

const MESSAGE_ENDING_BYTE: &str = "\x1E";

#[derive(Deserialize, Serialize, Debug)]
pub struct TransportDefinition {
    #[serde(rename = "transport")]
    transport_name: String,
    #[serde(rename = "transferFormats")]
    transport_format: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NegotiateRequest {
    #[serde(rename = "connectionToken")]
    pub token: String,
    #[serde(rename = "connectionId")]
    id: String,
    #[serde(rename = "negotiateVersion")]
    negotiate_version: u16,
    #[serde(rename = "availableTransports")]
    available_transports: Vec<TransportDefinition>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NegotiateResposne {
    protocol: String,
    version: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SocketMessage {
    #[serde(rename = "type")]
    req_type: u32,
    target: Option<String>,
    arguments: Option<Vec<String>>,
    #[serde(rename = "invocationId")]
    invocation_id: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Handshake {    
    Request { format: String, version: u32 },
    Response {
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String> 
    },
}

#[derive(Debug)]
pub enum Messsage {
    Invocation(InvocationFields), //type = 1
    StreamItem(StreamItemFields), // type = 2
    Completion(CompletionFields),//type = 3
    StreamInvocation(StreamInvocationFields), //type = 4
    CancelInvokation(CancelInvokationFields), //type = 5
    Ping, //type = 6
    Close(CloseFields),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvocationFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>, 
    target: String, 
    arguments: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamItemFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>, 
    item: serde_json::Value 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>, 
    result: serde_json::Value 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamInvocationFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>, 
    target: String, 
    arguments: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CancelInvokationFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CloseFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String> 
}

impl Messsage {
    pub fn deserialize(json: &str) -> Option<Self> {
        let result = serde_json::from_str::<Self>(json);
        if let Ok(obj) = result {
            Some(obj)
        } else {
            None
        }
    }

    pub fn serialize(self) -> Option<String> {
        if let Ok(json) = serde_json::to_string(&self) {
            let json = json + MESSAGE_ENDING_BYTE;
            Some(json)
        } else {
            None
        }
    }
}

// Taken from https://stackoverflow.com/questions/65575385/deserialization-of-json-with-serde-by-a-numerical-value-as-type-identifier/65576570#65576570
impl Serialize for Messsage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum MessageVariants_<'a> {
            Invocation(&'a InvocationFields), //type = 1
            StreamItem(&'a StreamItemFields), // type = 2
            Completion(&'a CompletionFields),//type = 3
            StreamInvocation(&'a StreamInvocationFields), //type = 4
            CancelInvokation(&'a CancelInvokationFields), //type = 5
            #[allow(dead_code)]
            Ping, //type = 6, not used to serialize but here for parity with outer type.
            Close(&'a CloseFields),
        }

        #[derive(Serialize)]
        struct TypedMessage<'a> {
            #[serde(rename = "type")]
            t: u64,
            #[serde(flatten, skip_serializing_if = "Option::is_none")]
            msg: Option<MessageVariants_<'a>>,
        }

        let msg = match self {
            Messsage::Invocation(fields) => TypedMessage { t: 1, msg: Some(MessageVariants_::Invocation(fields)) },
            Messsage::StreamItem(fields) => TypedMessage { t: 2, msg: Some(MessageVariants_::StreamItem(fields)) },
            Messsage::Completion(fields) => TypedMessage { t: 3, msg: Some(MessageVariants_::Completion(fields)) },
            Messsage::StreamInvocation(fields) => TypedMessage { t: 4, msg: Some(MessageVariants_::StreamInvocation(fields)) },
            Messsage::CancelInvokation(fields) => TypedMessage { t: 5, msg: Some(MessageVariants_::CancelInvokation(fields)) },
            Messsage::Ping => TypedMessage { t: 6, msg: None },
            Messsage::Close(fields) => TypedMessage { t: 7, msg: Some(MessageVariants_::Close(fields)) },
        };
        msg.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Messsage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = Value::deserialize(deserializer)?;
        Ok(match value.get("type").and_then(Value::as_u64).unwrap() {
            1 => Messsage::Invocation(InvocationFields::deserialize(value).unwrap()),
            2 => Messsage::StreamItem(StreamItemFields::deserialize(value).unwrap()),
            3 => Messsage::Completion(CompletionFields::deserialize(value).unwrap()),
            4 => Messsage::StreamInvocation(StreamInvocationFields::deserialize(value).unwrap()),
            5 => Messsage::CancelInvokation(CancelInvokationFields::deserialize(value).unwrap()),
            6 => Messsage::Ping,
            7 => Messsage::Close(CloseFields::deserialize(value).unwrap()),
            type_ => panic!("Unsupported type {:?}", type_),
        })
    }
}
