use crate::error::Result;
use crate::tcp::protocol::{DBCommands, pack_command};
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct KVSClient {
    stream: TcpStream,
}

impl KVSClient {
    /// Create server connection
    pub fn new(addr: String) -> Result<Self> {
        let stream = TcpStream::connect(addr)?;
        Ok(KVSClient { stream })
    }

    /// send command to server
    pub fn send_cmd(&mut self, command: DBCommands) -> Result<String> {
        
        let cmd_packet = pack_command(command)?;
        let _ = &self.stream.write_all(&cmd_packet)?;
        let _ = &self.stream.flush()?;

        let mut buf = String::new();
        let _ = &self.stream.read_to_string(&mut buf)?;
        Ok(buf)
    }
}
