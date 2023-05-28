use std::net::{TcpListener};
use std::sync::Arc;
use std::thread;
use log::{error, info};
use crate::server::book::{BookService};
use crate::error::Error;

pub struct Server {
    address: String,
    book_service: Arc<BookService>,
}

impl Server {
    pub fn new(address: String, book_service: Arc<BookService>) -> Server {
        Server { address, book_service }
    }

    pub fn start(&self) -> Result<(), Error> {
        let listener = TcpListener::bind(&self.address).unwrap();
        info!("Server listening on {}", self.address);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let book_service = self.book_service.clone();
                    thread::spawn(move || {
                        match book_service.handle_request(stream) {
                            Ok(_) => { info!("Finish request") }
                            Err(e) => { error!("Error: {}", e) }
                        };
                    });
                }
                Err(e) => {
                    error!("Failed to accept a connection: {}", e);
                }
            }
        }
        Ok(())
    }
}
