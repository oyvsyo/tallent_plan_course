use std::io::Read;
use std::net::{TcpListener, TcpStream};
use crate::engine::KvsEngine;
use crate::error::{Result, KVSError};


pub struct KvsServer<S: KvsEngine> {
    listener: TcpListener,
    store: S
}

impl<S: KvsEngine> KvsServer<S> {

    /// Creates new server object
    pub fn new(addr: String, store: S) -> Result<Self> {
        
        let listener = TcpListener::bind(addr).unwrap();
        let obj = KvsServer {
            listener,
            store
        };
        Ok(obj)
    }
    pub fn listen(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
    
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
    
        stream.read(&mut buffer).unwrap();
    
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
    
}