use std::env;
use std::sync::Arc;
use log::error;
use word_of_wisdom_pow::server::store::BookStore;
use word_of_wisdom_pow::common::pow::PowProvider;
use word_of_wisdom_pow::server::book::BookService;
use word_of_wisdom_pow::server::server::Server;

fn main() {
    env_logger::init();
    let address = env::var("HOST").unwrap_or("127.0.0.1:8001".to_string());
    let book_store = BookStore::new(vec![
        "The only true wisdom is in knowing you know nothing".to_string(),
        "It does not matter how slowly you go as long as you do not stop".to_string(),
        "In the middle of every difficulty lies opportunity".to_string(),
    ]);
    let pow_provider = PowProvider::new(2);
    let book_service = Arc::new(BookService::new(book_store, pow_provider));
    let server = Server::new(address, book_service);
    match server.start() {
        Ok(_) => {}
        Err(e) => {
            error!("Failed to start server: {}", e)
        }
    }
}
