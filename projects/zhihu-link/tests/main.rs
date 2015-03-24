use std::{io::Write, str::FromStr};
use zhihu_link::ZhihuAnswer;

#[test]
fn ready() {
    println!("it works!")
}

// #[tokio::test]
// async fn test_reqwest() {
//     let answer = ZhihuAnswer::from_str(include_str!("../test.html")).unwrap();
//     answer.save("test.md").await.unwrap();
// }
//
// #[tokio::test]
// async fn test_reqwest2() {
//     let answer = ZhihuAnswer::request(347662352, 847873806).await.unwrap();
//     let mut file = std::fs::File::create("test.html").unwrap();
//     file.write_all(answer.as_bytes()).unwrap();
// }
