mod errors;

use std::io::Write;
pub use errors::{Error, Result};
use scraper::Html;

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
    let out = Html::parse_document(include_str!("test.html")).unwrap();
    for node in out.children.iter() {
        println!("{:#?}", out.children.get(1));
    }
}