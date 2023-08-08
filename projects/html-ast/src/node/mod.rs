mod display;

use crate::HtmlElement;
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

#[derive(Clone, PartialEq, Eq)]
pub enum HtmlNode {
    /// A doctype.
    Doctype(DocType),
    /// A comment.
    Comment(String),
    /// Text.
    Text(Cow<'static, str>),
    /// An element.
    Element(HtmlElement),
    /// A processing instruction.
    ProcessingInstruction(ProcessingInstruction),
}

/// A doctype.
#[derive(Clone, PartialEq, Eq)]
pub struct DocType {
    /// The doctype name.
    pub name: String,
    /// The doctype public ID.
    pub public_id: String,
    /// The doctype system ID.
    pub system_id: String,
}

/// HTML Processing Instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessingInstruction {
    /// The PI target.
    pub target: String,
    /// The PI data.
    pub data: String,
}
