use chat::ChatMessage;
use chrono::Utc;
use futures::StreamExt;
use tokio::io::{self, AsyncBufReadExt};
use tokio::io::BufReader;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
pub mod chat {
    tonic::include_proto!("chat");
}
use chat::chat_service_client::ChatServiceClient;
use tonic::transport::Channel;
use tonic::Request;

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

    let response = client.chat_stream(Request::new(in_stream)).await.unwrap();
    let mut response_stream = response.into_inner();
    while let Some(rec) = response_stream.next().await {
        let item = rec.unwrap();
        println!("Received {:?} at {:?}", item.message, item.timestamp);
    }
}


#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match ChatServiceClient::connect("http://[::1]:50051").await {
        Ok(mut client) => {
            println!("✅ Connected to server!");
            chat(&mut client).await;
        }, Err(_) => {
            println!("Server disconnected")
        }
    }
    Ok(())
}