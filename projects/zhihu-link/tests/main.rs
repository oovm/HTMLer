use std::{io::Write, str::FromStr};
use zhihu_link::{utils::save_string, BilibiliArticle, ZhihuAnswer, ZhihuArticle, ZhihuAuto};

#[test]
fn ready() {
    println!("it works!")
}

#[tokio::test]
async fn test_reqwest() {
    let answer = BilibiliArticle::from_str(include_str!("../test_bilibili.html")).unwrap();
    answer.save("test.md").unwrap();
}

#[tokio::test]
async fn test_reqwest2() {
    let answer = ZhihuAnswer::request(347662352, 847873806).await.unwrap();
    save_string("test_answer.html", &answer).unwrap();
    let request = ZhihuArticle::request(438085414).await.unwrap();
    save_string("test_article.html", &request).unwrap();
    let article = BilibiliArticle::request(403592).await.unwrap();
    save_string("test_bilibili.html", &article).unwrap();
}

#[tokio::test]
async fn test_url() {
    // let answer = ZhihuAuto::new("https://www.zhihu.com/question/30928007/answer/1360071170").unwrap();
    let answer = ZhihuAuto::new("https://zhuanlan.zhihu.com/p/438085414").await.unwrap();
    let mut file = std::fs::File::create("test.md").unwrap();
    file.write_all(answer.as_bytes()).unwrap();
    // answer.save("test.md").await.unwrap();
}
