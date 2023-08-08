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
    pub fn with_text<S: ToString>(mut self, text: S) -> Self {
        self.children.push(HtmlNode::Text(Cow::Owned(text.to_string())));
        self
    }
}
