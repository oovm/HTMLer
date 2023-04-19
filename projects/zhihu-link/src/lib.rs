#![feature(try_blocks)]
#![feature(once_cell)]

mod answers;
mod auto;
mod bilibili;
mod errors;
pub mod utils;
mod zhuanlans;

pub use crate::{answers::ZhihuAnswer, auto::AutoMarkdown, bilibili::article::BilibiliArticle, zhuanlans::ZhihuArticle};
pub use errors::{MarkResult, ZhihuError};
