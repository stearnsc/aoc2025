use std::fmt::{Display, Formatter};
pub use log::{trace, debug, info, warn, error};
pub use winnow::{self, Parser};
pub use anyhow;

pub fn init() {
    dotenv::dotenv().expect(".env not found");
    pretty_env_logger::init();
}

#[derive(Debug)]
pub struct ParseError(pub String);

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse error: {}", self.0)
    }
}

impl std::error::Error for ParseError {}