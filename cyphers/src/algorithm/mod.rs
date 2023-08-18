use anyhow::Result;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait AlgorithmStrategy {
    fn encrypt(&self, message: &str) -> Result<String>;
    fn decrypt(&self, cypher: &str) -> Result<String>;
}

mod cesar;
pub use cesar::Cesar;

mod vernam;
pub use vernam::Vernam;

#[enum_dispatch(AlgorithmStrategy)]
pub enum Algorithm {
    Cesar,
    Vernam,
}
