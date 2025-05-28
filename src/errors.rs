use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MathError {
    #[error("Division by zero")]
    DivisionByZero,

    #[error("Positif integer required")]
    PositifIntegerRequired,

    #[error("Overflow")]
    Overflow,

    #[error("Out of range")]
    OutOfRange,
}
