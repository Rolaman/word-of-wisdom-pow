use std::io::{BufReader, BufWriter, Error, Read, Write};
use std::net::{Shutdown, TcpStream};
use log::info;
use crate::common::pow::{PowError, PowProvider, PowSolution};
use crate::server::store::BookStore;

pub struct BookService {
    store: BookStore,
    pow: PowProvider,
}

#[derive(thiserror::Error, Debug)]
pub enum BookError {
    #[error("invalid message format")]
    InvalidClientMessageError,
    #[error("failed to check solution: {0}")]
    SolutionCheckError(#[from] PowError),
    #[error("no quotes in storage")]
    NoQuotesAvailable,
    #[error("failed to write quote: {0}")]
    StreamError(#[from] Error),
}

impl BookService {
    pub fn new(store: BookStore, pow: PowProvider) -> BookService {
        BookService { store, pow }
    }

    pub fn handle_request(&self, stream: TcpStream) -> Result<(), BookError> {
        let reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        let challenge = self.pow.generate_challenge()?;

        writer.write_all(challenge.as_slice())?;
        writer.flush()?;
        info!("Challenge has been sent");
        match self.process_response(reader, challenge[..8].to_vec()) {
            Ok(_) => { info!("Challenge has been solved") }
            Err(err) => {
                stream.shutdown(Shutdown::Both)?;
                info!("Server shutdown");
                return Err(err);
            }
        };
        let quote = self.store.get_random_quote().ok_or(BookError::NoQuotesAvailable)?;
        writer.write_all(quote.as_bytes()).map_err(Into::into)
    }

    fn process_response(&self, mut reader: BufReader<&TcpStream>, original_challenge: Vec<u8>) -> Result<(), BookError> {
        let mut buffer = [0; 16];
        reader.read_exact(&mut buffer)?;
        if buffer.len() != 16 {
            return Err(BookError::InvalidClientMessageError);
        }
        let challenge = buffer[8..].to_vec();
        if original_challenge != challenge {
            return Err(BookError::InvalidClientMessageError);
        }
        let nonce = u64::from_le_bytes(buffer[..8].to_vec().try_into().unwrap());
        let solution = PowSolution::new(challenge, nonce);
        self.pow.check_solution(&solution).map_err(Into::into)
    }
}