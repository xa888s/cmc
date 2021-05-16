use thiserror::Error;

pub type CrackmeResult<T> = Result<T, CrackmeError>;

/// The primary error type for crackme related errors
// TODO: use an enum instead of just strings
#[derive(Error, Debug)]
pub enum CrackmeError {
    #[error("No value found for {0}!")]
    NotFound(&'static str),

    #[error("Failed to parse {0}!")]
    DetailParse(&'static str),
}
