use std::error::Error;

pub mod aws;
#[cfg(feature = "gzip")]
pub mod gzip;
pub mod log;
pub mod request;
#[cfg(feature = "sqs")]
pub mod status;

#[cfg(feature = "sqs")]
pub use aws::{lambda, queue};

pub type HandlerError = Box<dyn Error + Send + Sync + 'static>;
