use thiserror::Error;

pub type CrackMeResult<T> = Result<T, CrackMeError>;

/// The primary error type for crackme related errors
// TODO: use an enum instead of just strings
#[derive(Error, Debug)]
pub enum CrackMeError {
    #[error("No value found for {0}!")]
    NotFound(&'static str),

    #[error("Failed to parse {0}!")]
    DetailParse(&'static str),
}
