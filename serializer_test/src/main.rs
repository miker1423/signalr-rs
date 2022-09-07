use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug)]
enum MessageVariants {
    Invocation(InvocationFields), //type = 1
    StreamItem(StreamItemFields), // type = 2
    Completion(CompletionFields),//type = 3
    StreamInvocation(StreamInvocationFields), //type = 4
    CancelInvokation(CancelInvokationFields), //type = 5
    Ping, //type = 6
    Close(CloseFields),
}

#[derive(Serialize, Deserialize, Debug)]
struct InvocationFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>, 
    target: String, 
    arguments: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamItemFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>, 
    item: serde_json::Value 
}

#[derive(Serialize, Deserialize, Debug)]
struct CompletionFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>, 
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>, 
    result: serde_json::Value 
}

#[derive(Serialize, Deserialize, Debug)]
struct StreamInvocationFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>, 
    target: String, 
    arguments: serde_json::Value
}

#[derive(Serialize, Deserialize, Debug)]
struct CancelInvokationFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    invocation_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
struct CloseFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String> 
}

// Taken from https://stackoverflow.com/questions/65575385/deserialization-of-json-with-serde-by-a-numerical-value-as-type-identifier/65576570#65576570
impl Serialize for MessageVariants {
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
            MessageVariants::Invocation(fields) => TypedMessage { t: 1, msg: Some(MessageVariants_::Invocation(fields)) },
            MessageVariants::StreamItem(fields) => TypedMessage { t: 2, msg: Some(MessageVariants_::StreamItem(fields)) },
            MessageVariants::Completion(fields) => TypedMessage { t: 3, msg: Some(MessageVariants_::Completion(fields)) },
            MessageVariants::StreamInvocation(fields) => TypedMessage { t: 4, msg: Some(MessageVariants_::StreamInvocation(fields)) },
            MessageVariants::CancelInvokation(fields) => TypedMessage { t: 5, msg: Some(MessageVariants_::CancelInvokation(fields)) },
            MessageVariants::Ping => TypedMessage { t: 6, msg: None },
            MessageVariants::Close(fields) => TypedMessage { t: 7, msg: Some(MessageVariants_::Close(fields)) },
        };
        msg.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for MessageVariants {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        let value = Value::deserialize(deserializer)?;
        Ok(match value.get("type").and_then(Value::as_u64).unwrap() {
            1 => MessageVariants::Invocation(InvocationFields::deserialize(value).unwrap()),
            2 => MessageVariants::StreamItem(StreamItemFields::deserialize(value).unwrap()),
            3 => MessageVariants::Completion(CompletionFields::deserialize(value).unwrap()),
            4 => MessageVariants::StreamInvocation(StreamInvocationFields::deserialize(value).unwrap()),
            5 => MessageVariants::CancelInvokation(CancelInvokationFields::deserialize(value).unwrap()),
            6 => MessageVariants::Ping,
            7 => MessageVariants::Close(CloseFields::deserialize(value).unwrap()),
            type_ => panic!("Unsupported type {:?}", type_),
        })
    }
}

fn main() {
    let fields = InvocationFields { 
        invocation_id: Some("1".to_owned()), 
        target: "SendMessage".to_owned(), 
        arguments: Value::Null
    };
    let invoke = MessageVariants::Invocation(fields);
    let json = serde_json::json!(invoke).to_string();
    println!("JSON: {}", json);
    let json = serde_json::json!(MessageVariants::Ping).to_string();
    println!("JSON: {}", json);

    let test_json = "{\"type\":1,\"target\":\"ReceiveMessage\",\"arguments\":[\"Hello\"]}";
    let obj = serde_json::from_str::<MessageVariants>(test_json);
    if let Ok(message) = obj {
        println!("OBJ {:?}", message);
    } else {
        println!("Error! {}", obj.unwrap_err());
    }
    let test_json = "{\"type\": 6}";
    let obj = serde_json::from_str::<MessageVariants>(test_json);
    if let Ok(message) = obj {
        println!("OBJ {:?}", message);
    } else {
        println!("Error! {}", obj.unwrap_err());
    }
}
