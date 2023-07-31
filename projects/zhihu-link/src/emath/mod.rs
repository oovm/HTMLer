use crate::{utils::select_text, MarkResult, ZhihuError};
use htmler::{Html, Node, NodeKind, Selector};
use std::{
    fmt::{Display, Formatter, Write},
    io::Write as _,
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
pub struct EMathDissussion {
    title: String,
    content: String,
}

impl Default for EMathDissussion {
    fn default() -> Self {
        Self { title: "".to_string(), content: "".to_string() }
    }
}

impl Display for EMathDissussion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "# {}\n\n{}", self.title, self.content)
    }
}

impl FromStr for EMathDissussion {
    type Err = ZhihuError;

    fn from_str(html: &str) -> Result<Self, Self::Err> {
        let mut empty = Self::default();
        empty.do_parse(html)?;
        Ok(empty)
    }
}

impl EMathDissussion {
    /// 通过问题 ID 和回答 ID 获取知乎回答, 并渲染为 markdown
    ///
    /// # Examples
    ///
    /// ```
    /// # use zhihu_link::ZhihuAnswer;
    /// let answer = ZhihuAnswer::new(58151047, 1).await?;
    /// ```
    pub async fn new(article: usize, page: usize) -> MarkResult<Self> {
        let html = Self::request(article, page).await?;
        Ok(html.parse()?)
    }
    pub async fn request(article: usize, page: usize) -> MarkResult<String> {
        let url = format!("https://bbs.emath.ac.cn/thread-{article}-{page}-0.html");
        let resp = reqwest::Client::new().get(url).send().await?;
        Ok(resp.text().await?)
    }
    pub fn save<P>(&self, path: P) -> MarkResult<()>
    where
        P: AsRef<Path>,
    {
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.to_string().as_bytes())?;
        Ok(())
    }
    fn do_parse(&mut self, html: &str) -> MarkResult<()> {
        let html = Html::parse_document(html);
        self.extract_title(&html)?;
        self.extract_description(&html)?;
        self.extract_content(&html)?;
        Ok(())
    }
    fn extract_title(&mut self, html: &Html) -> MarkResult<()> {
        let selector = Selector::new("span#thread_subject");
        self.title = select_text(html, &selector).unwrap_or_default();
        Ok(())
    }
    fn extract_description(&mut self, html: &Html) -> MarkResult<()> {
        let selector = Selector::new("div.QuestionRichText");
        let _: Option<_> = try {
            for node in html.select(&selector) {
                let text = node.first_child()?.as_text()?;
                println!("text: {:?}", text);
            }
        };
        Ok(())
    }
    fn extract_content(&mut self, html: &Html) -> MarkResult<()> {
        // div.RichContent-inner
        let selector = Selector::new("td.t_f");
        for post in html.select(&selector) {
            for child in post.children() {
                self.read_content_node(child)?;
            }
            write!(self.content, "\n\n---\n")?;
        }
        Ok(())
    }
    fn read_content_node(&mut self, node: Node) -> MarkResult<()> {
        match node.as_kind() {
            NodeKind::Document => {
                println!("document")
            }
            NodeKind::Fragment => {
                println!("fragment")
            }
            NodeKind::Doctype(_) => {
                println!("doctype")
            }
            NodeKind::Comment(_) => {
                println!("comment")
            }
            NodeKind::Text(t) => {
                self.content.push_str(t.trim());
            }
            NodeKind::Element(e) => {
                match e.name() {
                    "div" => {
                        if e.has_class("attach_tips") {
                            // do nothing
                        }
                        else if e.has_class("quote") {
                            // do nothing
                        }
                        else if e.has_class("blockcode") {
                            self.extract_code_block(node)?
                        }
                        else {
                            println!("{:?}", e);
                            println!("{:?}", node.text().collect::<String>())
                        }
                    }
                    "p" => {
                        for child in node.children() {
                            self.read_content_node(child)?;
                        }
                        self.content.push_str("\n\n");
                    }
                    // "span" => {
                    //     // math mode
                    //     if e.has_class("ztext-math") {
                    //         match e.get_attribute("data-tex") {
                    //             Some(s) => {
                    //                 self.content.push_str(" $$");
                    //                 self.content.push_str(s);
                    //                 self.content.push_str("$$ ");
                    //             }
                    //             None => {}
                    //         }
                    //     }
                    //     // normal mode
                    //     else {
                    //         for child in node.children() {
                    //             self.read_content_node(child)?;
                    //         }
                    //     }
                    // }
                    "br" => {
                        self.content.push_str("\n");
                    }
                    "i" => {
                        if e.has_class("pstatus") {
                            // do nothing
                        }
                        else {
                            println!("{:?}", e);
                            println!("{:?}", node.text().collect::<String>())
                        }
                    }
                    "a" => match e.get_attribute("href") {
                        Some(s) => {
                            write!(self.content, "[{}]({})", node.text().collect::<String>(), s)?;
                        }
                        _ => {}
                    },
                    _ => {
                        println!("{:?}", e);
                        println!("{:?}", node.text().collect::<String>())
                    }
                    // "figure" => {
                    //     for child in node.descendants().filter(|e| e.has_class("img")) {
                    //         let original = child.get_attribute("data-original");
                    //         if !original.is_empty() {
                    //             write!(self.content, "![]({})", original)?;
                    //             break;
                    //         }
                    //     }
                    // }
                    unknown => panic!("unknown element: {unknown}"),
                }
            }
            NodeKind::ProcessingInstruction(_) => {
                println!("processing instruction");
            }
        }
        Ok(())
    }
    fn extract_code_block(&mut self, node: Node) -> MarkResult<()> {
        let selector = Selector::new("li");
        write!(self.content, "```rust\n")?;
        for child in node.select(&selector) {
            match child.first_child().and_then(|s| s.as_text()) {
                Some(s) => write!(self.content, "{}\n", s.trim())?,
                None => {}
            }
        }
        write!(self.content, "```")?;
        Ok(())
    }
}
