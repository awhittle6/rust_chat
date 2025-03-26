

use std::env;
use std::time::Duration;

use chat::ChatMessage;
use chrono::Utc;
use futures::StreamExt;
use tokio::io::{self, AsyncBufReadExt};
use tokio::io::BufReader;
use tokio::time::sleep;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
pub mod chat {
    tonic::include_proto!("chat");
}
use chat::chat_service_client::ChatServiceClient;
use tonic::transport::Channel;
use tonic::Request;
use dotenv::dotenv;

const MAX_RETRIES : i8 = 5;


pub async fn input() -> String {
    println!("Type something...");
    let mut inp = String::new();
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    reader.read_line(& mut inp).await.expect("Failed to read line");
    inp.trim().to_string()
}

async fn chat(client: &mut ChatServiceClient<Channel>){
    let (tx, rx) = mpsc::channel(128);
    let in_stream = ReceiverStream::new(rx);

    tokio::spawn(async move {
        // let thread = thread::current();
        // println!("Name: {:?}, id: {:?}", thread.name(), thread.id());
        loop {
            
            let user_msg = input().await;
            if user_msg.eq_ignore_ascii_case("exit") {
                break;
            } else {
                let msg = ChatMessage {
                    message: user_msg.to_string(),
                    timestamp: Utc::now().timestamp(),
                };

                if tx.send(msg).await.is_err() {
                    break;
                }
            }
        }
    });
    // let t = client.chat_stream(Request::new(in_stream)).await;
    // s
    // if let Ok(t) = client.chat_stream(Request::new(in_stream)).await {

    // }

    match client.chat_stream(Request::new(in_stream)).await {
        Ok(res) => {
            let mut response_stream = res.into_inner();
            while let Some(rec) = response_stream.next().await {
                // let message = rec.unwrap();
                if let Err(_) = rec {

                } 
                match rec {
                    Ok(message) => {
                        println!("{}", message.message);
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


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let mut retries : i8 = 0;
    let t = env::var("SERVER_ENDPOINT");
    println!("{t:?}");
    let server_location = std::env::var("SERVER_ENDPOINT").unwrap();
    while retries < MAX_RETRIES {
        println!("Attempt {} of connecting to the server at {} ...", retries + 1, &server_location);
        match ChatServiceClient::connect(server_location.clone()).await {
            Ok(mut client) => {
                println!("✅ Connected to server!");
                chat(&mut client).await;
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