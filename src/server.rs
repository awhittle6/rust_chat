use chrono::Utc;
use tonic::Streaming;
use tonic::{transport::Server, Status, Request, Response};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::{Stream, StreamExt};
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::sync::Arc;
use tokio_stream::wrappers::ReceiverStream;
pub mod chat {
    tonic::include_proto!("chat");
}

use chat::chat_service_server::{ChatService, ChatServiceServer};
use chat::ChatMessage;


#[derive(Debug, Default)]
pub struct MyChatService {
    clients: Arc<Mutex<HashMap<String, mpsc::Sender<Result<ChatMessage, Status>>>>>,
}

type ResponseStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;
type ChatResult<T> = Result<Response<T>, Status>;

#[tonic::async_trait]
impl ChatService for MyChatService {
    type ChatStreamStream = ResponseStream;

    async fn chat_stream(&self, request: Request<Streaming<ChatMessage>>) -> 
    ChatResult<Self::ChatStreamStream> {
        let client_id = request.remote_addr().map(|addr| addr.to_string()).unwrap();
        let (tx, rx) = mpsc::channel(128);
        {
            let mut clients = self.clients.lock().await;
            println!("Current number of clients in the server: {}", clients.len());
            clients.insert(client_id.clone(), tx.clone());
        }
        let mut incoming = request.into_inner();
        let clients = self.clients.clone();
        
        tokio::spawn(async move {
            while let Some(result) = incoming.next().await {
                match result {
                    Ok(res) => {
                        println!("{}: {:?} \n{:?}\n\n", &client_id,res.message, res.timestamp );
                        let clients = clients.lock().await;
                        for (id, client_tx) in clients.iter() {
                            if id != &client_id {
                                let message = ChatMessage {
                                    message: format!("{}: {:?} \n{:?}\n\n", &client_id,res.message, res.timestamp),
                                    timestamp: Utc::now().timestamp()
                                };
                                client_tx.send(Ok(message)).await.unwrap();
                            }
                        }
                        
                    }, 
                    Err(_) => {
                        eprintln!("Chat session ended!");
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
    let server = MyChatService {
        clients: Arc::new(Mutex::new(HashMap::new())),
    };
    Server::builder()
        .add_service(ChatServiceServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();

    println!("Server started on port");
    
    Ok(())
}