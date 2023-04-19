use std::{io::Write, str::FromStr};
use zhihu_link::{utils::save_string, AutoMarkdown, BilibiliArticle, ZhihuAnswer, ZhihuArticle};

#[test]
fn ready() {
    println!("it works!")
}

#[ignore]
#[tokio::test]
async fn export_bilibili() {
    let input = std::fs::read_to_string("test_bilibili.html").unwrap();
    let answer = BilibiliArticle::from_str(&input).unwrap();
    answer.save("tests/bilibili/cv4079473.md").unwrap();
}

#[ignore]
#[tokio::test]
async fn pre_fetch() {
    let answer = ZhihuAnswer::request(347662352, 847873806).await.unwrap();
    save_string("test_answer.html", &answer).unwrap();
    let request = ZhihuArticle::request(438085414).await.unwrap();
    save_string("test_article.html", &request).unwrap();
    let article = BilibiliArticle::request(4079473).await.unwrap();
    save_string("test_bilibili.html", &article).unwrap();
}

#[tokio::test]
async fn test_url() {
    // let answer = ZhihuAuto::new("https://www.zhihu.com/question/30928007/answer/1360071170").unwrap();
    let answer = AutoMarkdown::new("https://zhuanlan.zhihu.com/p/438085414").await.unwrap();
    let mut file = std::fs::File::create("test.md").unwrap();
    file.write_all(answer.as_bytes()).unwrap();
    // answer.save("test.md").await.unwrap();
}
