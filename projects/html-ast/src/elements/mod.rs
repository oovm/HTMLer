mod display;
use super::*;
use crate::node::HtmlNode;
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

/// An HTML element.
#[derive(Clone, PartialEq, Eq)]
pub struct HTMLElement {
    tag: String,
    id: String,
    classes: Vec<String>,
    attributes: BTreeMap<String, String>,
    children: Vec<HtmlNode>,
}

impl Default for HTMLElement {
    fn default() -> Self {
        Self { tag: "html".to_string(), id: "".to_string(), classes: vec![], attributes: Default::default(), children: vec![] }
    }
}

impl HTMLElement {
    pub fn new<S: ToString>(tag: S) -> Self {
        Self { tag: tag.to_string(), ..Default::default() }
    }
    pub fn with_id<S: ToString>(self, id: S) -> Self {
        Self { id: id.to_string(), ..self }
    }
    pub fn with_class<S: ToString>(mut self, class: S) -> Self {
        self.classes.push(class.to_string());
        self
    }
    pub fn with_attribute<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        self.attributes.insert(key.to_string(), value.to_string());
        self
    }
    pub fn with_child<H: Into<HtmlNode>>(mut self, child: H) -> Self {
        self.children.push(child);
        self
    }
    pub fn with_text<S: ToString>(mut self, text: S) -> Self {
        self.children.push(HtmlNode::Text(text.to_string()));
        self
    }
}
