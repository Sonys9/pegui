use alloc::string::String;
use thiserror::Error;

/// Errors
#[derive(Error, Debug)]
pub enum Error {
    /// Happens when something failed to get something (like the UI failed to find the font by the tag)
    #[error("Get fail: {0}")]
    FailedToGet(String),

    /// Happens when something failed to draw
    #[error("Draw error: {0}")]
    DrawError(String),

    /// Happens when something failed to send something
    #[error("Send error: {0}")]
    SendError(String),

    /// Happens when something failed to receive something
    #[error("Receive error: {0}")]
    ReceiveError(String),

    /// Uses to stop the update loop
    #[error("Aborted")]
    End(()),

    /// For not listed errors
    #[error("Unknown error: {0}")]
    UnknownError(String),
}
