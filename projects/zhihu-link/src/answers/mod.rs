use crate::{ZhihuError, ZhihuResult};
use std::{
    fmt::{Display, Formatter},
    path::Path,
    str::FromStr,
};

use ego_tree::NodeRef;
use htmler::{Html, Node, Selector};
use tokio::io::AsyncWriteExt;

#[derive(Debug)]
pub struct ZhihuAnswer {
    title: String,
    content: String,
}

impl Default for ZhihuAnswer {
    fn default() -> Self {
        Self { title: "".to_string(), content: "".to_string() }
    }
}

impl Display for ZhihuAnswer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "# {}\n\n{}", self.title, self.content)
    }
}

impl FromStr for ZhihuAnswer {
    type Err = ZhihuError;

    fn from_str(html: &str) -> Result<Self, Self::Err> {
        let mut empty = Self::default();
        empty.do_parse(html)?;
        Ok(empty)
    }
}

impl ZhihuAnswer {
    pub async fn new(question: usize, answer: usize) -> ZhihuResult<Self> {
        let html = Self::request(question, answer).await?;
        Ok(html.parse()?)
    }
    pub async fn request(question: usize, answer: usize) -> ZhihuResult<String> {
        let url = format!("https://www.zhihu.com/question/{question}/answer/{answer}");
        let resp = reqwest::Client::new().get(url).send().await?;
        Ok(resp.text().await?)
    }
    pub async fn save<P>(&self, path: P) -> ZhihuResult<()>
    where
        P: AsRef<Path>,
    {
        let mut file = tokio::fs::File::create(path).await?;
        file.write_all(self.to_string().as_bytes()).await?;
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
        let selector = Selector::new("h1.QuestionHeader-title");
        let _: Option<_> = try {
            let node = html.select(&selector).next()?;
            let text = node.first_child()?.value().as_text()?;
            self.title = text.to_string();
        };
        Ok(())
    }
    fn extract_description(&mut self, html: &Html) -> ZhihuResult<()> {
        let selector = Selector::new("div.QuestionRichText");
        let _: Option<_> = try {
            for node in html.select(&selector) {
                let text = node.first_child()?.value().as_text()?;
                println!("text: {:?}", text);
            }
        };
        Ok(())
    }
    fn extract_content(&mut self, html: &Html) -> ZhihuResult<()> {
        // div.RichContent-inner
        let selector = Selector::new("span.CopyrightRichText-richText");
        let _: Option<_> = try {
            let node = html.select(&selector).next()?;
            for child in node.children() {
                self.read_content_node(child).ok()?;
            }
        };
        Ok(())
    }
    fn read_content_node(&mut self, node: NodeRef<Node>) -> ZhihuResult<()> {
        match node.value() {
            Node::Document => {
                println!("document")
            }
            Node::Fragment => {
                println!("fragment")
            }
            Node::Doctype(_) => {
                println!("doctype")
            }
            Node::Comment(_) => {
                println!("comment")
            }
            Node::Text(t) => {
                self.content.push_str(t.trim());
            }
            Node::Element(e) => {
                match e.name() {
                    "p" => {
                        for child in node.children() {
                            self.read_content_node(child)?;
                        }
                        self.content.push_str("\n\n");
                    }
                    "span" => {
                        // math mode
                        if e.has_class("ztext-math") {
                            for child in node.first_children() {
                                println!("child: {:?}", child.value());
                                match child.value().as_element() {
                                    Some(s) => {
                                        println!("element: {:?}", s);
                                    }
                                    _ => {}
                                }

                                // match child.value().as_element() {
                                //     Some(s) if s.is_a("script") => {
                                //         for class in e.classes() {
                                //             println!("class: {}", class);
                                //         }
                                //     }
                                //     _ => {}
                                // }
                            }
                            match e.get_attribute("data-tex") {
                                Some(s) => {
                                    self.content.push_str(" $$");
                                    self.content.push_str(s);
                                    self.content.push_str("$$ ");
                                }
                                None => {}
                            }
                        }
                        // normal mode
                        else {
                            for child in node.children() {
                                self.read_content_node(child)?;
                            }
                        }
                    }
                    "br" => {
                        self.content.push_str("\n");
                    }
                    "figure" => {
                        for child in node.descendants() {
                            // data-original
                        }
                    }
                    unknown => panic!("unknown element: {unknown}"),
                }
            }
            Node::ProcessingInstruction(_) => {
                println!("processing instruction");
            }
        }
        Ok(())
    }
}
