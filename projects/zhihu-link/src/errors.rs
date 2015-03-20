#[derive(Debug, Copy, Clone)]
pub enum ZhihuError {
    UnknownError,
}

pub type ZhihuResult<T> = std::result::Result<T, ZhihuError>;

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
