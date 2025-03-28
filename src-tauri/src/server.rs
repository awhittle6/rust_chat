use chrono::Utc;
use tonic::Streaming;
use tonic::{transport::Server, Status, Request, Response};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::{Stream, StreamExt};
use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::pin::Pin;
use std::sync::Arc;
use dotenv::dotenv;
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
            clients.insert(client_id.clone(), tx.clone());
            println!("LOG: {} has joined the server", &client_id);
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
                                    message: res.clone().message,
                                    from: res.clone().from,
                                    timestamp: Utc::now().timestamp()
                                };
                                client_tx.send(Ok(message)).await.unwrap();
                            }
                        }
                        
                    }, 
                    Err(_) => {
                        println!("DEBUG: {} has left the server", client_id);
                        break;
                    }
                }
            }
            let mut clients = clients.lock().await;
            clients.remove(&client_id);
        });
        let rc = ReceiverStream::new(rx);
        let f = Box::pin(rc) as Self::ChatStreamStream;
        Ok(Response::new(f))
    }
    // async fn chat_stream<'a>(&'a self, request: Request<tonic::Streaming<ChatMessage>>) -> Result<Response<Self::ChatStreamStream>, Status> {

    // }
}

// type ResponseStream = std::pin::Pin<dyn Stream>

// #[tokio::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>>{
    dotenv().ok();
    let server = MyChatService {
        clients: Arc::new(Mutex::new(HashMap::new())),
    };
    let port = std::env::var("PORT").unwrap_or_else(|_| "50051".to_string());
    let addr = format!("0.0.0.0:{}", port);
    Server::builder()
    .add_service(ChatServiceServer::new(server))
    .serve(addr.to_socket_addrs().unwrap().next().unwrap())
    .await.unwrap();
    
    Ok(())
}