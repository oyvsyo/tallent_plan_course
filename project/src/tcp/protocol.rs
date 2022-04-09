use serde::{Deserialize, Serialize};
use clap::{Subcommand};

use crate::engine::KvsEngine;
// use crate::error::{Result, KVSError};

// const MAX_PACKET_LENGTH: usize = 1024; 
// pub type COMMAND_LEN_TYPE = u64;


#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[derive(Subcommand)]
pub enum DBCommands {
    /// Set up value by key into KVS
    Set { key: String, value: String },
    /// Get value by key
    Get { key: String },
    /// Removes value by key
    Rm { key: String },
}


impl DBCommands {

    /// Invoke command on KvsEngine and return string result
    pub fn invoke_cmd<S: KvsEngine>(&self, store: &mut S) -> String {
        match self {
            DBCommands::Get { key } => {
                if let Ok(res) = store.get(key.to_owned()) {
                    match res {
                        Some(v) => v.clone(),
                        None => String::from("Key not found")
                    }
                } else {
                    String::from("Error")
                }
            },
            DBCommands::Set { key, value } => {
                if let Ok(_res) = store.set(key.to_owned(), value.to_owned()) {
                    String::new()
                } else {
                    String::from("Cant set")
                }
            },
            DBCommands::Rm { key } => {
                if let Ok(_res) = store.remove(key.to_owned()) {
                    String::new()
                } else {
                    String::from("Key not found")
                }
            }
        }
    }
}

// pub fn command_to_bytes(command: DBCommands) -> Result<Vec<u8>> {
//     let cmd_str = serde_json::to_string(&command)?;
//     let cmd_length = cmd_str.len() as COMMAND_LEN_TYPE;
//     let mut packet = cmd_length.to_be_bytes().to_vec();
//     packet.extend(cmd_str.as_bytes());
//     Ok(packet)
// }

// pub fn command_from_bytes(bytes: Vec<u8>) -> Result<DBCommands> {
//     let hui = &bytes[0..8];
//     let k = hui.try_into().;
//     let len_bytes = hui;
//     let command_len = u64::from_be_bytes(len_bytes);
//     let cmd = serde_json::from_slice(bytes)
// }