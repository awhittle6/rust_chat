use std::str::FromStr;
use std::time::Duration;

use chat::ChatMessage;
use chrono::Utc;
use futures::StreamExt;
use tokio::io::{self, AsyncBufReadExt};
use tokio::io::BufReader;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{Sender, Receiver};
pub mod chat {
    tonic::include_proto!("chat");
}
use chat::chat_service_client::ChatServiceClient;
use tonic::{transport::Channel, Request, Status, Response};
use dotenv::dotenv;

const MAX_RETRIES : i8 = 5;

// static CLIENT: Mutex<Option<ChatServiceClient<tonic::transport::Channel>>> = Mutex::const_new(None);
// static TX: Mutex<Option<mpsc::Sender<Result<ChatMessage, tonic::Status>>>> = Mutex::const_new(None);
// static STREAM: Mutex<Option<tokio_stream<ChatMessage>>> = Mutex::const_new(None);

pub type MutexOptional<T> = Mutex<Option<T>>;
pub type MessageResult<T> = Result<Response<T>, Status>;




static CLIENT : MutexOptional<ChatServiceClient<Channel>> = Mutex::const_new(None);
static TX: MutexOptional<Sender<ChatMessage>> = Mutex::const_new(None);
static STREAM: MutexOptional<ReceiverStream<ChatMessage>> = Mutex::const_new(None);

pub async fn input(prompt: Option<String>) -> String {
    if let Some(prompt) = prompt {
        println!("{prompt}");
    }
    let mut inp = String::new();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    reader.read_line(& mut inp).await.expect("Failed to read line");
    inp.trim().to_string()
}



pub async fn send_message(content: String) -> Result<(), Box<dyn std::error::Error>> {
    let tx_guard = TX.lock().await;
    if let Some(tx) = &*tx_guard {
        let message = ChatMessage {
            from: "".to_string(),
            message: content,
            timestamp: Utc::now().timestamp(),
        };
        match tx.send(Ok(message)).await {
            Ok(_) => Ok(()),
            Err(_) => Err("Error".into()),
        }
    } else {
        Err("Error".into())
    }
}


pub async fn handle_incoming_stream() {
    let mut stream_guard = STREAM.lock().await;
    if let Some(stream) = &mut *stream_guard{
        while let Some(result) = stream.next().await {
            println!("Result: {:?}", result);
        }
    }
}
async fn chat(client: &mut ChatServiceClient<Channel>){
    let (tx, rx) = mpsc::channel(128);
    let in_stream = ReceiverStream::new(rx);

    tokio::spawn(async move {
        // let name = input(Some("Please enter your name:".to_string())).await;
        loop {
            let user_msg = input(None).await;
            if user_msg.eq_ignore_ascii_case("exit") {
                break;
            } else {
                let msg = ChatMessage {
                    message: user_msg.to_string(),
                    from: "".to_string(),
                    timestamp: Utc::now().timestamp(),
                };
            }
        }
    });

    match client.chat_stream(Request::new(in_stream)).await {
        Ok(res) => {
            let mut response_stream = res.into_inner();
            while let Some(rec) = response_stream.next().await {
                // let message = rec.unwrap();
                match rec {
                    Ok(message) => {
                        println!("{}: {}",message.from, message.message);
                    },
                    Err(_) => {
                        println!("You've been disconnected from the server");
                        break;
                    }
                }
            }
        },
        Err(e) => {
            println!("Status error in chat stream: {e}");
        }
    }
    // switch client.chat_stream(Request::new(in_stream)).await {

    // }
    // let response = client.chat_stream(Request::new(in_stream)).await.unwrap();
    // let mut response_stream = response.into_inner();
    // while let Some(rec) = response_stream.next().await {
    //     let item = rec.unwrap();
    //     println!("Received {:?} at {:?}", item.message, item.timestamp);
    // }
}



pub async fn join_server(addr: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut retries : i8 = 0;
    let server_location = std::string::String::from_str(addr).unwrap();
    while retries < MAX_RETRIES {
        println!("Attempt {} of connecting to the server at {} ...", retries + 1, &server_location);
        match ChatServiceClient::connect(server_location.clone()).await {
            Ok(mut client) => {
                *CLIENT.lock().await = Some(client);
                let (tx, rx) = mpsc::channel(128);
                *STREAM.lock().await = Some(ReceiverStream::new(rx));
                *TX.lock().await = Some(tx);
                println!("✅ Connected to server!");
                break;
            }, Err(_) => {
                retries += 1;
                if retries == MAX_RETRIES {
                    println!("Server not available.");
                    break;
                }
            }
        }
        sleep(Duration::from_millis(1500)).await;
    }
    Ok(())
}