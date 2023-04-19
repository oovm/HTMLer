use std::io::Write;
use zhihu_link::{ZhihuAnswer, ZhihuAuto};

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
#[tokio::test]
async fn test_url() {
    // let answer = ZhihuAuto::new("https://www.zhihu.com/question/30928007/answer/1360071170").unwrap();
    let answer = ZhihuAuto::new("https://zhuanlan.zhihu.com/p/438085414").await.unwrap();
    let mut file = std::fs::File::create("test.md").unwrap();
    file.write_all(answer.as_bytes()).unwrap();
    // answer.save("test.md").await.unwrap();
}
