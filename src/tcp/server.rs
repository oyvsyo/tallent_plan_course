use crate::engine::KvsEngine;
use crate::error::Result;
use crate::tcp::protocol::DBCommands;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

pub struct KvsServer<S: KvsEngine> {
    addr: String,
    store: S,
}

impl<S: KvsEngine> KvsServer<S> {
    /// Creates new server object with KvsEngine object
    pub fn new(addr: String, store: S) -> Result<Self> {
        let obj = KvsServer { addr, store };
        log::info!("Version -- {}", env!("CARGO_PKG_VERSION"));
        log::info!("Created KVSStore successful");
        Ok(obj)
    }
    /// Run listener for incomming requests
    pub fn listen(&mut self) {
        let listener = TcpListener::bind(&self.addr).unwrap();
        log::info!("Running Server on {}", &self.addr);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    match self.handle_connection(stream) {
                        Ok(_) => log::info!("Client command served"),
                        Err(e) => log::error!("{}", e),
                    };
                }
                Err(e) => {
                    log::error!("{}", e)
                }
            }
        }
    }
    /// here parse request, invoke command by engine and return response
    fn handle_connection(&mut self, mut stream: TcpStream) -> Result<()> {
        let cmd = DBCommands::from_stream(&mut stream)?;
        log::debug!("Command - {:?}", cmd);
        let resp = cmd.invoke_cmd(&mut self.store);
        log::debug!("Result - {:?}", resp);
        let resp_bytes = resp.to_packet()?;
        stream.write_all(&resp_bytes)?;
        stream.flush()?;
        Ok(())
    }
}
