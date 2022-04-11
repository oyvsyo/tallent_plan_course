use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::io::Read;

use crate::engine::KvsEngine;
use crate::error::{KVSError, Result};

const CMD_HEAD: &'static [u8] = &[27, 59];
const LEN_SIZE: usize = 4; 
// const MAX_PACKET_LENGTH: usize = 1024;
pub type CommandLenType = u32;

#[derive(Debug, Serialize, Deserialize, Subcommand)]
pub enum DBCommands {
    /// Set up value by key into KVS
    Set { key: String, value: String },
    /// Get value by key
    Get { key: String },
    /// Removes value by key
    Rm { key: String },
}

const GET_BYTE: u8 = 1;
const SET_BYTE: u8 = 2;
const RM_BYTE: u8 = 3;


impl DBCommands {
    /// Invoke command on KvsEngine and return string result
    pub fn invoke_cmd<S: KvsEngine>(&self, store: &mut S) -> String {
        match self {
            DBCommands::Get { key } => {
                if let Ok(res) = store.get(key.to_owned()) {
                    match res {
                        Some(v) => v.clone(),
                        None => String::from("Key not found"),
                    }
                } else {
                    String::from("Error")
                }
            }
            DBCommands::Set { key, value } => {
                if let Ok(_res) = store.set(key.to_owned(), value.to_owned()) {
                    String::new()
                } else {
                    String::from("Cant set")
                }
            }
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

pub fn pack_command(command: DBCommands) -> Result<Vec<u8>> {
    let (cmd, key, value) = match command {
        DBCommands::Get { key } => (GET_BYTE, key, String::from("")),
        DBCommands::Rm { key } => (RM_BYTE, key, String::from("")),
        DBCommands::Set { key, value } => (SET_BYTE, key, value),
    };
    let k_len: CommandLenType = key.len().try_into().unwrap();
    let v_len: CommandLenType = value.len().try_into().unwrap();

    let k_len_enc = k_len.to_be_bytes().to_vec();
    let v_len_enc = v_len.to_be_bytes().to_vec();
    let mut cmd_vec = Vec::new();
    cmd_vec.push(cmd);
    let packet = [
        CMD_HEAD.to_vec(),
        cmd_vec,
        k_len_enc,
        v_len_enc,
        key.into_bytes(),
        value.into_bytes(),
    ]
    .concat();
    Ok(packet)
}

pub fn unpack_command(stream: &mut TcpStream) -> Result<DBCommands> {

    let mut head = [0u8; 2];
    &stream.read_exact(&mut head)?;

    if CMD_HEAD[0] != head[0] || CMD_HEAD[0] != head[0] {
        return Err(KVSError::GeneralKVSError)
    }
    
    let mut cmd = [0u8; 1];
    &stream.read_exact(&mut cmd)?;

    let cmd = cmd[0];

    let mut key_len_coded = [0u8; LEN_SIZE];
    stream.read_exact(&mut key_len_coded)?;
    let mut val_len_coded = [0u8; LEN_SIZE];
    stream.read_exact(&mut val_len_coded)?;
    let key_len = CommandLenType::from_be_bytes(key_len_coded);
    let val_len = CommandLenType::from_be_bytes(val_len_coded);

    let mut key = vec![0u8; key_len as usize];
    stream.read_exact(&mut key)?;
    let key = String::from_utf8_lossy(&key).into_owned();
    let mut value = vec![0u8; val_len as usize];
    stream.read_exact(&mut value)?;
    let value = String::from_utf8_lossy(&value).into_owned();

    match cmd {
        GET_BYTE => Ok(DBCommands::Get { key }),
        SET_BYTE => Ok(DBCommands::Set { key, value }),
        RM_BYTE => Ok(DBCommands::Rm { key }),
        _ => Err(KVSError::GeneralKVSError)
    }
}