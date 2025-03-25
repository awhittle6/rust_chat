use chrono::Utc;
use tonic::Streaming;
use tonic::{transport::Server, Status, Request, Response};
use tokio::sync::mpsc;
use tokio_stream::{Stream, StreamExt};
use std::net::ToSocketAddrs;
use std::pin::Pin;
use tokio_stream::wrappers::ReceiverStream;
pub mod chat {
    tonic::include_proto!("chat");
}

use chat::chat_service_server::{ChatService, ChatServiceServer};
use chat::ChatMessage;


#[derive(Debug, Default)]
pub struct MyChatService {}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;
type ChatResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl ChatService for MyChatService {
    type ChatStreamStream = ResponseStream;

    async fn chat_stream(&self, request: Request<Streaming<ChatMessage>>) -> 
    ChatResult<Self::ChatStreamStream> {
        let mut incoming = request.into_inner();
        let (tx, rx) = mpsc::channel(128);

        tokio::spawn(async move {
            while let Some(result) = incoming.next().await {
                match result {
                    Ok(res) => {
                        println!("Received: {:?} at {:?}", res.message, Utc::now().timestamp());
                        tx.send(Ok(res)).await.unwrap();
                    }, 
                    Err(e) => {
                        eprintln!("Error: {e:?}");
                        break;
                    }
                }
            }
        });
        let rc = ReceiverStream::new(rx);
        let f = Box::pin(rc) as Self::ChatStreamStream;
        Ok(Response::new(f))
    }
    // async fn chat_stream<'a>(&'a self, request: Request<tonic::Streaming<ChatMessage>>) -> Result<Response<Self::ChatStreamStream>, Status> {

    // }
}

// type ResponseStream = std::pin::Pin<dyn Stream>

#[tokio::main]
pub async fn main () -> Result<(), Box<dyn std::error::Error>>{
    let server = MyChatService {};
    println!("Server started");
    Server::builder()
        .add_service(ChatServiceServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    Ok(())
}