use std::error::Error;
use std::fmt::{Display, Formatter};
pub use log::{trace, debug, info, warn, error};
pub use winnow::{self, Parser};
pub use anyhow;

pub fn init() {
    dotenv::dotenv().expect(".env not found");
    pretty_env_logger::init();
}

#[derive(Debug)]
pub struct TextError(pub String);


impl Display for TextError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl std::error::Error for TextError {}

pub trait BoxedError: Error + Send + 'static + Sized {
    fn boxed(self) -> Box<dyn Error + Send + 'static> {
        Box::new(self)
    }
}

impl<E> BoxedError for E where E: Error + Send + 'static + Sized {

}

pub trait BoxedResult<T, E>: Sized where E: BoxedError {
    fn boxed(self) -> Result<T, Box<dyn Error + Send + 'static>>;
}

impl<T, E> BoxedResult<T, E> for Result<T, E> where E: BoxedError {
    fn boxed(self) -> Result<T, Box<dyn Error + Send + 'static>> {
        self.map_err(|e| e.boxed())
    }
}