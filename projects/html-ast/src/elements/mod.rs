mod display;

use crate::node::HtmlNode;
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fmt::{Display, Formatter},
};

/// An HTML element.
#[derive(Clone, PartialEq, Eq)]
pub struct HtmlElement {
    tag: String,
    id: String,
    classes: BTreeSet<String>,
    attributes: BTreeMap<String, String>,
    children: Vec<HtmlNode>,
}

impl Default for HtmlElement {
    fn default() -> Self {
        Self {
            tag: "html".to_string(),
            id: "".to_string(),
            classes: BTreeSet::default(),
            attributes: Default::default(),
            children: vec![],
        }
    }
}

impl HtmlElement {
    /// Create a new HTML element with the given tag.
    pub fn new<S: ToString>(tag: S) -> Self {
        Self { tag: tag.to_string(), ..Default::default() }
    }
    pub fn set_tag<S: ToString>(&mut self, tag: S) {
        self.tag = tag.to_string();
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn set_id<S: ToString>(&mut self, id: S) {
        self.id = id.to_string();
    }
    pub fn with_id<S: ToString>(self, id: S) -> Self {
        Self { id: id.to_string(), ..self }
    }
    pub fn get_classes(&self) -> &BTreeSet<String> {
        &self.classes
    }
    pub fn mut_classes(&mut self) -> &mut BTreeSet<String> {
        &mut self.classes
    }
    pub fn add_class<S: ToString>(&mut self, class: S) {
        self.classes.insert(class.to_string());
    }
    pub fn with_class<S: ToString>(mut self, class: S) -> Self {
        self.add_class(class);
        self
    }
    pub fn get_attributes(&self) -> &BTreeMap<String, String> {
        &self.attributes
    }
    pub fn mut_attributes(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.attributes
    }
    pub fn add_attribute<K: ToString, V: ToString>(&mut self, key: K, value: V) {
        self.attributes.insert(key.to_string(), value.to_string());
    }
    pub fn with_attribute<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        self.add_attribute(key, value);
        self
    }
    pub fn get_children(&self) -> &[HtmlNode] {
        &self.children
    }
    pub fn mut_children(&mut self) -> &mut Vec<HtmlNode> {
        &mut self.children
    }
    pub fn add_child<H: Into<HtmlNode>>(&mut self, child: H) {
        self.children.push(child.into());
    }
    pub fn with_child<H: Into<HtmlNode>>(mut self, child: H) -> Self {
        self.add_child(child);
        self
    }
    /// Add a section of HTML text that does not need to be transferred
    pub fn add_safe_text(&mut self, text: Cow<'static, str>) {
        self.children.push(HtmlNode::Text(text))
    }
    /// Add a section of HTML text and make righteousness
    pub fn add_text<S: AsRef<str>>(&mut self, text: S) {
        let txt = text.as_ref();
        let mut text = String::with_capacity(txt.len());
        for c in txt.chars() {
            match c {
                '<' => text.push_str("&lt;"),
                '>' => text.push_str("&gt;"),
                '&' => text.push_str("&amp;"),
                '"' => text.push_str("&quot;"),
                '\'' => text.push_str("&#39;"),
                _ => text.push(c),
            }
        }
        self.children.push(HtmlNode::Text(Cow::Owned(text)));
    }
    pub fn with_safe_text(mut self, text: Cow<'static, str>) -> Self {
        self.add_safe_text(text);
        self
    }
    pub fn with_text<S: AsRef<str>>(mut self, text: S) -> Self {
        self.add_text(text);
        self
    }
}
