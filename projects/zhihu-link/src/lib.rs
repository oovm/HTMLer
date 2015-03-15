#![feature(try_blocks)]

mod errors;
mod answers;

use std::io::Write;

pub use errors::{Error, ZhihuResult};


pub use crate::answers::ZhihuAnswer;

#[tokio::test]
async fn test_reqwest() {
    let url = "https://www.zhihu.com/question/{}/answer/{}";
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await.unwrap();
    let text = resp.text().await.unwrap();
    // write to file
    let mut file = std::fs::File::create("test.html").unwrap();
    file.write_all(text.as_bytes()).unwrap();
}

#[test]
fn test_parse() {
    let mut zhihu = ZhihuAnswer::new();
    zhihu.parse(include_str!("test.html")).unwrap();
    println!("zhihu: {:#?}", zhihu);
}
