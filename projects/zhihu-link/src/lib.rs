#![feature(try_blocks)]
#![feature(once_cell)]

mod answers;
mod auto;
mod bilibili;
mod errors;
pub mod utils;
mod zhuanlans;

pub use crate::{answers::ZhihuAnswer, auto::ZhihuAuto, bilibili::article::BilibiliArticle, zhuanlans::ZhihuArticle};
pub use errors::{ZhihuError, ZhihuResult};
