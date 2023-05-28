use std::env;
use log::{error, info};
use word_of_wisdom_pow::client::client::Client;

fn main()  {
    env_logger::init();
    let address = env::var("HOST").unwrap_or("127.0.0.1:8001".to_string());
    let mut client = Client::new(address.as_str());

    match client.connect() {
        Ok(_) => {}
        Err(e) => {
            info!("Can't connect to server {:?}", e);
            return;
        }
    }

    match client.fetch_word() {
        Ok(w) => {
            info!("Word of wisdom: {}", w);
        }
        Err(e) => {
            error!("Can't fetch word of wisdom: {:?}", e);
        }
    }
}
