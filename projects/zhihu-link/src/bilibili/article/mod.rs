use crate::{utils::select_text, ZhihuError, ZhihuResult};
use htmler::{Html, Node, NodeKind, Selector};
use std::{
    fmt::{Display, Formatter, Write},
    io::Write as _,
    path::Path,
    str::FromStr,
    sync::LazyLock,
};

#[derive(Debug)]
pub struct BilibiliArticle {
    title: String,
    content: String,
}

impl Default for BilibiliArticle {
    fn default() -> Self {
        Self { title: "".to_string(), content: "".to_string() }
    }
}

impl Display for BilibiliArticle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "# {}\n\n{}", self.title, self.content)
    }
}

impl FromStr for BilibiliArticle {
    type Err = ZhihuError;

    fn from_str(html: &str) -> Result<Self, Self::Err> {
        let mut empty = Self::default();
        empty.do_parse(html)?;
        Ok(empty)
    }
}
static SELECT_TITLE: LazyLock<Selector> = LazyLock::new(|| Selector::new("h1.Post-Title"));
static SELECT_CONTENT: LazyLock<Selector> = LazyLock::new(|| Selector::new("script#js-initialData"));

// script#js-initialData

impl BilibiliArticle {
    /// 通过问题 ID 和回答 ID 获取知乎回答, 并渲染为 markdown
    ///
    /// # Examples
    ///
    /// ```
    /// # use zhihu_link::ZhihuAnswer;
    /// let answer = ZhihuAnswer::new(58151047, 1).await?;
    /// ```
    pub async fn new(article: usize) -> ZhihuResult<Self> {
        let html = Self::request(article).await?;
        Ok(html.parse()?)
    }
    pub async fn request(article: usize) -> ZhihuResult<String> {
        let url = format!("https://www.bilibili.com/read/cv{article}");
        let resp = reqwest::Client::new().get(url).send().await?;
        Ok(resp.text().await?)
    }
    pub fn save<P>(&self, path: P) -> ZhihuResult<()>
    where
        P: AsRef<Path>,
    {
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.to_string().as_bytes())?;
        Ok(())
    }
    fn do_parse(&mut self, html: &str) -> ZhihuResult<()> {
        let html = Html::parse_document(html);
        self.extract_title(&html)?;
        self.extract_description(&html)?;
        self.extract_content(&html)?;
        Ok(())
    }

    fn extract_title(&mut self, html: &Html) -> ZhihuResult<()> {
        self.title = select_text(&html, &SELECT_TITLE).unwrap_or_default();
        Ok(())
    }
    fn extract_description(&mut self, html: &Html) -> ZhihuResult<()> {
        let selector = Selector::new("div.QuestionRichText");
        let _: Option<_> = try {
            for node in html.select(&selector) {
                let text = node.first_child()?.as_text()?;
                println!("text: {:?}", text);
            }
        };
        Ok(())
    }
    fn extract_content(&mut self, html: &Html) -> ZhihuResult<()> {
        // div.RichContent-inner
        let json = select_text(&html, &SELECT_CONTENT).unwrap_or_default();
        let decode = serde_json::from_str::<serde_json::Value>(&json)?;
        self.content = format!("{:#?}", decode);
        Ok(())
    }
}
