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

    /// Happens when something failed to send somethind
    #[error("Send error: )")]
    SendError(String),
}
