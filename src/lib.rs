use std::error::Error;

pub mod aws;
#[cfg(feature = "gzip")]
pub mod gzip;
#[cfg(feature = "sqs")]
pub mod lambda;
pub mod log;
#[cfg(feature = "sqs")]
pub mod queue;
pub mod request;

pub type HandlerError = Box<dyn Error + Send + Sync + 'static>;