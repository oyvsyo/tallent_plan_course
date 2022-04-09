use std::net::TcpStream;
use std::io::{Read, Write};
use crate::tcp::protocol::{DBCommands};
use crate::error::{Result};


pub struct KVSClient {
    stream: TcpStream
}

impl KVSClient {
    /// Create server connection
    pub fn new(addr: String) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(KVSClient{stream})
    }

    /// send command to server
    pub fn send_cmd(&mut self, command: DBCommands) ->Result<String> {
    
        let cmd_str = serde_json::to_string(&command)?;
        // println!("send command: |{}|  {}", cmd_str, cmd_str.len());
        let _ = &self.stream.write_all(cmd_str.as_bytes())?;
        let _ = &self.stream.flush()?;
        
        let mut buf = String::new();
        let _ = &self.stream.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
