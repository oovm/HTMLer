#![feature(try_blocks)]

mod answers;
mod errors;

use std::io::Write;

pub use errors::{ZhihuError, ZhihuResult};

pub use crate::answers::ZhihuAnswer;

#[tokio::test]
async fn test_reqwest() {
    let answer = ZhihuAnswer::new(347662352, 847873806).await.unwrap();
    answer.save("test.md").await.unwrap();
}

#[tokio::test]
async fn test_reqwest2() {
    let answer = ZhihuAnswer::request(347662352, 847873806).await.unwrap();
    let mut file = std::fs::File::create("test.html").unwrap();
    file.write_all(answer.as_bytes()).unwrap();
}
