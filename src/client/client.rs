use std::io::{Read, Write};
use std::io::Error;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use log::info;
use crate::common::pow::{PowError, PowSolver};

pub struct Client {
    host: String,
    stream: Option<TcpStream>,
}

#[derive(thiserror::Error, Debug)]
pub enum ClientError {
    #[error("connection error: {0}")]
    ConnectError(#[from] Error),
    #[error("can't read server message: {0}")]
    ReadError(Error),
    #[error("no open stream available")]
    NoOpenStream,
    #[error("can't find nonce: {0}")]
    NoNonceFound(#[from] PowError),
    #[error("wrong solution")]
    WrongSolution,
}

impl Client {
    pub fn new(server_address: &str) -> Client {
        Client {
            host: server_address.to_string(),
            stream: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), ClientError> {
        self.handshake()?;
        let challenge = self.read_challenge()?;
        let nonce = self.solve(&challenge)?;
        self.send_solution(nonce, &challenge)?;
        info!("Connected");
        Ok(())
    }

    fn handshake(&mut self) -> Result<(), ClientError> {
        let mut server_socket = self.host.to_socket_addrs().map_err(ClientError::ConnectError)?;
        let ready_socket = server_socket
            .next()
            .ok_or_else(||
                ClientError::ConnectError(
                    Error::new(std::io::ErrorKind::Other, "Failed to resolve server address")
                ))?;
        let stream = TcpStream::connect(ready_socket).map_err(ClientError::ConnectError)?;
        stream.set_read_timeout(Some(Duration::from_secs(2))).map_err(ClientError::ConnectError)?;
        self.stream = Some(stream);
        info!("Connected to TcpStream: {}", self.host);
        Ok(())
    }

    fn read_challenge(&mut self) -> Result<Vec<u8>, ClientError> {
        let mut buffer = [0; 9];
        info!("Reading first message");
        match self.stream.as_mut() {
            Some(s) => {
                s.read(&mut buffer).map_err(ClientError::ReadError)?;
                info!("Got challenge with difficulty: {}", buffer[8]);
                Ok(buffer.to_vec())
            }
            None => {
                Err(ClientError::NoOpenStream)
            }
        }
    }

    fn solve(&self, challenge: &Vec<u8>) -> Result<u64, ClientError> {
        let solver = PowSolver::new(challenge[..8].to_vec(), challenge[8].clone());
        solver.find_nonce().map_err(Into::into)
    }

    fn send_solution(&mut self, nonce: u64, challenge: &Vec<u8>) -> Result<(), ClientError> {
        let mut buffer = nonce.to_le_bytes().to_vec();
        buffer.extend_from_slice(&challenge[..8]);
        match self.stream.as_mut() {
            Some(s) => {
                s.write_all(buffer.as_slice()).map_err(ClientError::ConnectError)?;
                s.flush().map_err(ClientError::ConnectError)?;
                Ok(())
            }
            None => {
                Err(ClientError::NoOpenStream)
            }
        }
    }

    pub fn fetch_word(&mut self) -> Result<String, ClientError> {
        match self.stream.as_mut() {
            Some(s) => {
                let word = &mut String::new();
                s.read_to_string(word).map_err(ClientError::ReadError)?;
                if word.is_empty() {
                    return Err(ClientError::WrongSolution)
                }
                Ok(word.clone())
            }
            None => {
                Err(ClientError::NoOpenStream)
            }
        }
    }
}
