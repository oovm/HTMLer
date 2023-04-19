#![feature(try_blocks)]
#![feature(once_cell)]

mod answers;
mod auto;
mod errors;
mod utils;
mod zhuanlans;

pub use crate::{answers::ZhihuAnswer, auto::ZhihuAuto, zhuanlans::ZhihuArticle};
pub use errors::{ZhihuError, ZhihuResult};
