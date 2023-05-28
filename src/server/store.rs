use rand::seq::SliceRandom;

pub struct BookStore {
    quotes: Vec<String>,
}

impl BookStore {
    pub fn new(quotes: Vec<String>) -> BookStore {
        BookStore { quotes }
    }

    pub fn get_random_quote(&self) -> Option<&String> {
        self.quotes.choose(&mut rand::thread_rng())
    }
}