use crate::error::Result;
use crate::tcp::protocol::{pack_command, unpack_response, DBCommands, ServerResponse};
use std::io::Write;
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
    pub fn send_cmd(&mut self, command: DBCommands) -> Result<ServerResponse> {
        let cmd_packet = pack_command(command)?;
        let _ = &self.stream.write_all(&cmd_packet)?;
        let _ = &self.stream.flush()?;

        unpack_response(&mut self.stream)
    }
}
