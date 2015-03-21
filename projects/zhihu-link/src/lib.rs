#![feature(try_blocks)]

mod answers;
mod errors;

pub use errors::{ZhihuError, ZhihuResult};

pub use crate::answers::ZhihuAnswer;
