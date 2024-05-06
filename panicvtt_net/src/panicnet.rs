use std::{io, net::Ipv4Addr, sync::{Arc, Mutex}};

use async_std::net::{TcpListener, TcpStream};
use tokio_util::sync::CancellationToken;
 
pub struct PanicNetServer {
    listener_token: CancellationToken,
    clients: Arc<Mutex<Vec<TcpStream>>>,
} 

impl PanicNetServer {

    async fn accept_incoming_connections(listener_token: CancellationToken, listener: TcpListener, connection_list: Arc<Mutex<Vec<TcpStream>>>) { 
        while !listener_token.is_cancelled() {
            let (_stream, _) = listener.accept().await.unwrap();
            println!("New client accepted!"); 

            connection_list.lock().unwrap().push(_stream);
        } 

        // !TODO this is never reached because listener.accept() never returns!
        println!("Incoming connection worker: listener token canceled! Goodnight.");
    }

    pub fn new() -> Self {
        Self {
            listener_token: CancellationToken::new(),
            clients: Arc::new(Mutex::new(Vec::new()))
        }
    } 

    pub async fn start(&self, ip: Ipv4Addr, port: u16) -> Result<(), io::Error>{ 
        let listener = TcpListener::bind((ip, port)).await?;
        let listener_token = self.listener_token.clone();
        
        tokio::spawn(Self::accept_incoming_connections(listener_token, listener, self.clients.clone())); 

        Ok(())
    }


    pub fn test(&self) {
        println!("I have {} connections", self.clients.lock().unwrap().len());
    }

    pub fn close(&self) {
        self.listener_token.cancel();
    }

    pub fn get_ip(&self) -> String {
        todo!()
    }

}

/*
let listener = TcpListener::bind((ip, port)).await?;
        let st = listener.accept().await?;
        let stream = st.0; 

        Result::Ok(Self {
            role: PanicNetRole::Server, 
            stream
        })
 */
