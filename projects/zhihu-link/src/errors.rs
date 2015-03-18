#[derive(Debug, Copy, Clone)]
pub enum Error {
    UnknownError,
}

pub type ZhihuResult<T> = std::result::Result<T, Error>;
