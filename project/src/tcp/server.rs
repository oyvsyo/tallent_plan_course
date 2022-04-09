use crate::engine::KvsEngine;
use crate::error::Result;
use crate::tcp::protocol::DBCommands;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

pub struct KvsServer<S: KvsEngine> {
    addr: String,
    store: S,
}

impl<S: KvsEngine> KvsServer<S> {
    /// Creates new server object with KvsEngine object
    pub fn new(addr: String, store: S) -> Result<Self> {
        let obj = KvsServer { addr, store };
        Ok(obj)
    }
    /// Run listener for incomming requests
    pub fn listen(&mut self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream).unwrap();
        }
    }
    /// here parse request, invoke command by engine and return response
    fn handle_connection(&mut self, mut stream: TcpStream) -> Result<()> {
        let mut client_buffer = [0u8; 1024];
        stream.read(&mut client_buffer)?;
        let cmd_str = String::from_utf8_lossy(&client_buffer);

        let trimmed = cmd_str.trim_matches(char::from(0));
        let cmd: DBCommands = serde_json::from_str(trimmed)?;

        let resp = cmd.invoke_cmd(&mut self.store);

        // println!(" res == {}", resp);
        stream.write_all(resp.as_bytes())?;
        stream.flush()?;
        Ok(())
    }
}
