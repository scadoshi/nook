use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum InvalidPost {
    #[error("missing account")]
    MissingAccount,
    #[error("missing amount")]
    MissingAmount,
    #[error(transparent)]
    ParseAmount(#[from] ParseIntError),
}

#[derive(Debug, Clone, Error)]
pub enum InvalidBalance {
    #[error("missing account")]
    Account,
}

#[derive(Debug, Clone, Error)]
pub enum InvalidCommand {
    #[error("input was empty")]
    Empty,
    #[error("unrecognized command")]
    Unrecognized,
    #[error("invalid post: {0}")]
    Post(#[from] InvalidPost),
    #[error("invalid balance: {0}")]
    Balance(#[from] InvalidBalance),
}
