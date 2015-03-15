use crate::ZhihuResult;

use scraper::{Html, Selector};

#[derive(Debug)]
pub struct ZhihuAnswer {
    title: String,
}

impl ZhihuAnswer {
    pub fn new() -> Self {
        Self { title: "".to_string() }
    }
    pub fn parse(&mut self, html: &str) -> ZhihuResult<()> {
        let html = Html::parse_document(html);

        self.extract_title(&html)?;
        self.extract_description(&html)?;
        self.extract_content(&html)?;


        Ok(())
    }
    pub fn extract_title(&mut self, html: &Html) -> ZhihuResult<()> {
        // #root > div > main > div > div > div:nth-child(10) > div:nth-child(2) > div > div.QuestionHeader-content > div.QuestionHeader-main > h1
        let selector = Selector::parse("h1.QuestionHeader-title").expect("invalid title selector");
        let _: Option<_> = try {
            let node = html.select(&selector).next()?;
            let text = node.first_child()?.value().as_text()?;
            self.title = text.to_string();
        };
        Ok(())
    }
    pub fn extract_description(&mut self, html: &Html) -> ZhihuResult<()> {
        // #root > div > main > div > div > div:nth-child(10) > div:nth-child(2) > div > div.QuestionHeader-content > div.QuestionHeader-main > div:nth-child(4) > div > div > div > div > span > p
        let selector = Selector::parse("div.QuestionRichText").expect("invalid description selector");
        let _: Option<_> = try {
            for node in html.select(&selector) {
                let text = node.first_child()?.value().as_text()?;
                println!("text: {:?}", text);
            }
        };
        Ok(())
    }
    pub fn extract_content(&mut self, html: &Html) -> ZhihuResult<()> {
        // #root > div > main > div > div > div.Question-main > div.ListShortcut > div > div.Card.AnswerCard.css-0 > div > div > div > div.RichContent.RichContent--unescapable > span:nth-child(1) > div > div > span
        let selector = Selector::parse("div.RichContent").expect("invalid content selector");
        let _: Option<_> = try {
            for node in html.select(&selector) {
                let text = node.first_child()?.value().as_text()?;
                println!("text: {:?}", text);
            }
        };
        Ok(())
    }
}