#[derive(Debug, Copy, Clone)]
pub enum ZhihuError {
    UnknownError,
}

pub type ZhihuResult<T> = Result<T, ZhihuError>;

impl From<reqwest::Error> for ZhihuError {
    fn from(_: reqwest::Error) -> Self {
        ZhihuError::UnknownError
    }
}

impl From<std::io::Error> for ZhihuError {
    fn from(_: std::io::Error) -> Self {
        ZhihuError::UnknownError
    }
}

impl From<std::fmt::Error> for ZhihuError {
    fn from(_: std::fmt::Error) -> Self {
        ZhihuError::UnknownError
    }
}

impl From<serde_json::Error> for ZhihuError {
    fn from(_: serde_json::Error) -> Self {
        ZhihuError::UnknownError
    }
}
