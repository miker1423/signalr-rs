use std::{sync::Arc, time::Duration, io::Error};

use signalr_rs::{
    HubConnectionBuilder, 
    error::Error as SignalRError,
    protocol::responses:: {
        Messsage as SignalRMessage,
        Handshake
    }
};
use tokio::{self, net::TcpStream, sync::{mpsc::{*, self}, Notify}, time};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use futures_util::{StreamExt, SinkExt, stream::{SplitStream, SplitSink}, future::{self, select, Either}};
use serde_json;

type SocketWriter = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;
type SocketReader = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type Socket = (SocketWriter, SocketReader);
type Pipe = (UnboundedSender<SignalRMessage>, UnboundedReceiver<SignalRMessage>);

async fn dispatcher_worker(
    mut invoker_ch: UnboundedReceiver<SignalRMessage>) {
    loop {
        if let Some(message) = invoker_ch.recv().await {
            println!("Dispatch message {:?}", message);
        }
    }
}

async fn read_ws(
    mut socket: SocketReader,
    inbound_ch: UnboundedSender<SignalRMessage>) {
    loop {
        if let Some(message) = socket.next().await {
            if let Ok(message) = message {
                match message {
                    Message::Text(txt) => {
                        if let Some(message) = SignalRMessage::deserialize(&txt) {
                            let _ = inbound_ch.send(message);
                        } else {
                            println!("Failed to parse");
                        }
                    },
                    _ => println!("The message is not text")
                }
            } else {
                println!("Something failed!!");
            }
            // Process message here! 
            // Deserialize into a SignalR message type and send to dispatcher.
            // Find multi-consumer channel
        } else {
            println!("Received none, we are closing");
            break;
        }
    }
}

async fn write_ws(
    mut socket: SocketWriter,
    mut outbound_ch: UnboundedReceiver<SignalRMessage>) {
    loop {
        if let Some(message) = outbound_ch.recv().await {
            if let Some(message) = message.serialize() {
                let _ = socket.send(Message::Text(message)).await;
            }
        } else {
            print!("Closed channel");
            break;
        }
    }
}

async fn heartbeat(
    outbound_ch: UnboundedSender<SignalRMessage>,
    notifier: Arc<tokio::sync::Notify>) {
    
    loop {
        let heartbeat = time::timeout(Duration::from_secs(2), notifier.notified());
        if let Err(_) = heartbeat.await {
            println!("send hearbeat!");
        } else {
            let result = outbound_ch.send(SignalRMessage::Ping);
            if let Err(error) = result {
                dbg!(println!("Found error {}", error));
            } else {                    
                dbg!("Sent ping message");
            }
            println!("message was sent, channel is ok");
        }
    }
}

const HANDSHAKE_MESSAGE: &str = "{\"protocol\":\"json\",\"version\":1}\x1E";

async fn do_handshake(mut ws: Socket) -> Result<Socket, SignalRError> {
    let socket_writer = &mut ws.0;
    let socket_reader = &mut ws.1;
    
    let message: Message = Message::Text(HANDSHAKE_MESSAGE.to_owned());
    let _ = socket_writer.send(message).await;
    if let Some(message) = socket_reader.next().await {
        if let Ok(message) = message {
            match message {
                Message::Text(json) => {
                    if let Ok(response) = serde_json::from_str::<signalr_rs::protocol::responses::Handshake>(&json) {
                        match response {
                            Handshake::Response { error } => {
                                if let Some(str) = error {
                                    Err(SignalRError::handshake_error(&str))
                                } else {
                                    Ok(ws)
                                }
                            },
                            _ => Err(SignalRError::handshake_error(""))
                        }
                    } else {
                        Err(SignalRError::handshake_error(""))
                    }
                },
                _ => Err(SignalRError::handshake_error(""))
            }
        } else {
            Err(SignalRError::handshake_error(""))
        }
    } else {
        Err(SignalRError::handshake_error(""))
    }

}

struct Builder {
    pref_serializer: String,
    prot_version: u32,
    
}

#[tokio::main]
async fn main() {
    let base = "localhost:5000/chat";
    let url = format!("http://{}", base);
    let connection = 
        HubConnectionBuilder::new()
                            .with_url(url.to_owned())
                            .build().await;
    if let None = connection {
        panic!("Failed to connect, cannot continue");        
    }
    let connection = connection.unwrap();
    let negotaite_url = format!("ws://{}?id={}", base, connection.token);

    let (ws, response) = connect_async(&negotaite_url).await.unwrap();
    let writer_channel = mpsc::unbounded_channel::<Message>();
    let (ch_writer, ch_read) = (writer_channel.0, writer_channel.1);
    let ch_writer = Arc::new(ch_writer);
    
    let outbound_ch = mpsc::unbounded_channel::<SignalRMessage>();
    let inbound_ch = mpsc::unbounded_channel::<SignalRMessage>();

    
    let notifier = Arc::new(Notify::new());

    let mut socket = ws.split();
    let hs_result = do_handshake(socket).await;
    let (socketWriter, socketReader) = 
        if let Ok(socket) = hs_result {
            socket
        } else {
            panic!("Handshake failded!");
        };
    let reader_ft = tokio::spawn(read_ws(socketReader, inbound_ch.0));
    let writer_ft = tokio::spawn(write_ws(socketWriter, outbound_ch.1));
    let heartbeat_ft = tokio::spawn(heartbeat(outbound_ch.0, notifier));
    let dispatcher_ft = tokio::spawn(dispatcher_worker(inbound_ch.1));
    
    let ab = future::select(reader_ft, writer_ft);
    let bc = future::select(ab, heartbeat_ft);
    future::select(bc, dispatcher_ft).await;
}
