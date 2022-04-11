use clap::Subcommand;
use crc16::{State, ARC};
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::net::TcpStream;

use crate::engine::KvsEngine;
use crate::error::{KVSError, Result};

const CMD_HEAD: &'static [u8] = &[27, 59];
const LEN_SIZE: usize = 4;

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
    pub fn invoke_cmd<S: KvsEngine>(&self, store: &mut S) -> ServerResponse {
        match self {
            DBCommands::Get { key } => {
                if let Ok(res) = store.get(key.to_owned()) {
                    match res {
                        Some(v) => ServerResponse::Success { output: v.clone() },
                        None => ServerResponse::Success {
                            output: String::from("Key not found"),
                        },
                        // None => ServerResponse::Failure{ message: String::from("Key not found")},
                    }
                } else {
                    ServerResponse::Failure {
                        message: String::from("Internal error"),
                    }
                }
            }
            DBCommands::Set { key, value } => {
                if let Ok(_res) = store.set(key.to_owned(), value.to_owned()) {
                    ServerResponse::Success {
                        output: String::from(""),
                    }
                } else {
                    ServerResponse::Failure {
                        message: String::from("Cant set"),
                    }
                }
            }
            DBCommands::Rm { key } => {
                if let Ok(_res) = store.remove(key.to_owned()) {
                    ServerResponse::Success {
                        output: String::new(),
                    }
                } else {
                    ServerResponse::Failure {
                        message: String::from("Key not found"),
                    }
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
    let checksum = State::<ARC>::calculate(&packet).to_be_bytes();
    let packet = [packet, checksum.to_vec()].concat();
    Ok(packet)
}

pub fn unpack_command(stream: &mut TcpStream) -> Result<DBCommands> {
    let mut head = [0u8; 2];
    let _ = &stream.read_exact(&mut head)?;

    if CMD_HEAD[0] != head[0] || CMD_HEAD[0] != head[0] {
        return Err(KVSError::GeneralKVSError);
    }

    let mut cmd_array = [0u8; 1];
    let _ = &stream.read_exact(&mut cmd_array)?;
    let cmd = cmd_array[0];

    let mut key_len_coded = [0u8; LEN_SIZE];
    stream.read_exact(&mut key_len_coded)?;
    let mut val_len_coded = [0u8; LEN_SIZE];
    stream.read_exact(&mut val_len_coded)?;
    let key_len = CommandLenType::from_be_bytes(key_len_coded);
    let val_len = CommandLenType::from_be_bytes(val_len_coded);

    let mut key_buff = vec![0u8; key_len as usize];
    stream.read_exact(&mut key_buff)?;
    let key = String::from_utf8_lossy(&key_buff).into_owned();
    let mut value_buff = vec![0u8; val_len as usize];
    stream.read_exact(&mut value_buff)?;
    let value = String::from_utf8_lossy(&value_buff).into_owned();

    // check hashsum of the data
    let mut checksum = vec![0u8; 2];
    stream.read_exact(&mut checksum)?;
    let packet = [
        CMD_HEAD.to_vec(),
        cmd_array.to_vec(),
        key_len_coded.to_vec(),
        val_len_coded.to_vec(),
        key_buff,
        value_buff,
    ]
    .concat();
    let calculated = State::<ARC>::calculate(&packet).to_be_bytes();
    if calculated[0] != checksum[0] && calculated[1] != checksum[1] {
        log::error!(
            "Checksum of command not matched, must be {:?}, received {:?}",
            calculated,
            checksum
        );
        return Err(KVSError::GeneralKVSError);
    }

    match cmd {
        GET_BYTE => Ok(DBCommands::Get { key }),
        SET_BYTE => Ok(DBCommands::Set { key, value }),
        RM_BYTE => Ok(DBCommands::Rm { key }),
        _ => Err(KVSError::GeneralKVSError),
    }
}

const SUCCESS_BYTE: u8 = 100;
const FAILURE_BYTE: u8 = 101;

#[derive(Debug)]
pub enum ServerResponse {
    Success { output: String },
    Failure { message: String },
}

pub fn pack_response(resp: ServerResponse) -> Result<Vec<u8>> {
    let (resp_byte, msg) = match resp {
        ServerResponse::Success { output } => (SUCCESS_BYTE, output),
        ServerResponse::Failure { message } => (FAILURE_BYTE, message),
    };
    let msg_len: CommandLenType = msg.len().try_into().unwrap();

    let msg_len_enc = msg_len.to_be_bytes().to_vec();
    let mut resp_vec = Vec::new();
    resp_vec.push(resp_byte);
    let packet = [CMD_HEAD.to_vec(), resp_vec, msg_len_enc, msg.into_bytes()].concat();
    let checksum = State::<ARC>::calculate(&packet).to_be_bytes();
    let packet = [packet, checksum.to_vec()].concat();
    Ok(packet)
}

pub fn unpack_response(stream: &mut TcpStream) -> Result<ServerResponse> {
    let mut head = [0u8; 2];
    let _ = &stream.read_exact(&mut head)?;

    if CMD_HEAD[0] != head[0] || CMD_HEAD[0] != head[0] {
        return Err(KVSError::GeneralKVSError);
    }

    let mut resp_byte = [0u8; 1];
    let _ = &stream.read_exact(&mut resp_byte)?;

    let resp_type = resp_byte[0];

    let mut msg_len_coded = [0u8; LEN_SIZE];
    stream.read_exact(&mut msg_len_coded)?;

    let msg_len = CommandLenType::from_be_bytes(msg_len_coded);

    let mut msg_vec = vec![0u8; msg_len as usize];
    stream.read_exact(&mut msg_vec)?;
    let msg = String::from_utf8_lossy(&msg_vec).into_owned();

    // check hashsum of the data
    let mut checksum = vec![0u8; 2];
    stream.read_exact(&mut checksum)?;
    let packet = [
        CMD_HEAD.to_vec(),
        resp_byte.to_vec(),
        msg_len_coded.to_vec(),
        msg_vec,
    ]
    .concat();
    let calculated = State::<ARC>::calculate(&packet).to_be_bytes();
    if calculated[0] != checksum[0] && calculated[1] != checksum[1] {
        log::error!(
            "Response checksum is not matched, must be {:?}, received {:?}",
            calculated,
            checksum
        );
        return Err(KVSError::GeneralKVSError);
    }

    match resp_type {
        SUCCESS_BYTE => Ok(ServerResponse::Success { output: msg }),
        FAILURE_BYTE => Ok(ServerResponse::Failure { message: msg }),
        _ => Err(KVSError::GeneralKVSError),
    }
}
