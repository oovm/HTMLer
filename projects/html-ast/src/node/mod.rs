use std::collections::BTreeMap;

pub enum HtmlNode {
    /// A doctype.
    Doctype(DocType),
    /// A comment.
    Comment(String),
    /// Text.
    Text(String),
    /// An element.
    Element(HTMLElement),
    /// A processing instruction.
    ProcessingInstruction(ProcessingInstruction),
}

impl Default for HtmlNode {
    fn default() -> Self {
        Self::Element(HTMLElement::default())
    }
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
