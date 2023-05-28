#[cfg(test)]
mod test {
    use std::net::TcpListener;
    use std::sync::Arc;
    use crate::server::book::BookService;
    use crate::client::client::Client;
    use crate::server::server::Server;
    use crate::common::pow::{check_solution, PowProvider};
    use crate::server::store::BookStore;

    #[test]
    fn test_pow_solver() {
        let challenge = [0, 0, 0, 0, 100, 111, 90, 117];
        let difficulty = 2;
        let invalid_nonce = 0;
        let valid_nonce = 145161;

        assert!(check_solution(invalid_nonce, &challenge.to_vec(), difficulty.clone()).is_err());
        assert!(check_solution(valid_nonce, &challenge.to_vec(), difficulty).is_ok());
    }

    #[test]
    fn integration_test() {
        //given
        let difficulty = 1;
        let store = BookStore::new(vec![
            "Test quote".to_string()
        ]);
        let pow_provider = PowProvider::new(difficulty);
        let port = find_free_port();
        let address = format!("127.0.0.1:{}", port);
        let mut client = Client::new(address.as_str());
        let book_service = Arc::new(BookService::new(store, pow_provider));
        let server = Server::new(address, book_service);

        //prep
        let _server_handle = std::thread::spawn(move || {
            server.start().expect("Error while starting server");
        });
        std::thread::sleep(std::time::Duration::from_secs(1));

        //test
        let result = client.connect();
        assert!(result.is_ok());
    }

    fn find_free_port() -> u16 {
        TcpListener::bind("127.0.0.1:0")
            .ok()
            .and_then(|listener| listener.local_addr().ok())
            .map(|socket_addr| socket_addr.port())
            .expect("No free port")
    }
}