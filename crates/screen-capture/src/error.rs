use std::sync::PoisonError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScreenCaptureError {
    #[error("{0}")]
    Error(String),
    #[error("StdSyncPoisonError {0}")]
    StdSyncPoisonError(String),
    #[error("Invalid capture region: {0}")]
    InvalidCaptureRegion(String),

    #[cfg(target_os = "windows")]
    #[error(transparent)]
    WindowsCoreError(#[from] windows::core::Error),
    #[cfg(target_os = "windows")]
    #[error(transparent)]
    Utf16Error(#[from] widestring::error::Utf16Error),
}

impl ScreenCaptureError {
    pub fn new<S: ToString>(err: S) -> Self {
        ScreenCaptureError::Error(err.to_string())
    }
}

pub type ScreenCaptureResult<T> = Result<T, ScreenCaptureError>;

impl<T> From<PoisonError<T>> for ScreenCaptureError {
    fn from(value: PoisonError<T>) -> Self {
        ScreenCaptureError::StdSyncPoisonError(value.to_string())
    }
}
