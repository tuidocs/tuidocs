#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("error while drawing the ui")]
    DrawError,
    #[error("error while reading events")]
    EventReadError,

    #[error("could not enable raw mode")]
    EnableRawModeError,
    #[error("could not disable raw mode")]
    DisableRawModeError,
    #[error("failed to create terminal")]
    TerminalCreationError,
    #[error("failed to show cursor")]
    ShowCursorError,

    #[error("unknown error: {0}")]
    UnknownError(String),
}