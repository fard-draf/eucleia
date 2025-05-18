use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MathError {
    #[error("Division by zero")]
    DivisionByZero,
}
