//! HTML nodes.

use fmt::Debug;
use std::cell::OnceCell;

use std::{fmt, ops::Deref, slice::Iter as SliceIter};

use crate::{CaseSensitivity, HtmlStr};
use html5ever::{Attribute, LocalName, QualName};
use indexmap::IndexMap;

/// An HTML node.
// `Element` is usually the most common variant and hence boxing it
// will most likely not improve performance overall.
#[allow(variant_size_differences)]
#[derive(Clone, PartialEq, Eq)]
pub enum NodeKind {
    /// The document root.
    Document,
    /// The fragment root.
    Fragment,
    /// A doctype.
    Doctype(Doctype),
    /// A comment.
    Comment(HtmlStr),
    /// Text.
    Text(HtmlStr),
    /// An element.
    Element(NodeData),
    /// A processing instruction.
    ProcessingInstruction(ProcessingInstruction),
}

impl NodeKind {
    /// Returns true if node is the document root.
    pub fn is_document(&self) -> bool {
        matches!(*self, NodeKind::Document)
    }

    /// Returns true if node is the fragment root.
    pub fn is_fragment(&self) -> bool {
        matches!(*self, NodeKind::Fragment)
    }

    /// Returns true if node is a doctype.
    pub fn is_doctype(&self) -> bool {
        matches!(*self, NodeKind::Doctype(_))
    }

    /// Returns true if node is a comment.
    pub fn is_comment(&self) -> bool {
        matches!(*self, NodeKind::Comment(_))
    }

    /// Returns true if node is text.
    pub fn is_text(&self) -> bool {
        matches!(*self, NodeKind::Text(_))
    }

    /// Returns true if node is an element.
    pub fn is_element(&self) -> bool {
        matches!(*self, NodeKind::Element(_))
    }

    /// Returns self as an element.
    pub fn as_element(&self) -> Option<&NodeData> {
        match *self {
            NodeKind::Element(ref e) => Some(e),
            _ => None,
        }
    }
}

// Always use one line.
impl Debug for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            NodeKind::Document => write!(f, "Document"),
            NodeKind::Fragment => write!(f, "Fragment"),
            NodeKind::Doctype(d) => write!(f, "Doctype({:?})", d),
            NodeKind::Comment(c) => write!(f, "Comment({:?})", c),
            NodeKind::Text(t) => write!(f, "Text({:?})", t),
            NodeKind::Element(e) => write!(f, "Element({:?})", e),
            NodeKind::ProcessingInstruction(pi) => write!(f, "ProcessingInstruction({:?})", pi),
        }
    }
}

/// A doctype.
#[derive(Clone, PartialEq, Eq)]
pub struct Doctype {
    /// The doctype name.
    pub name: HtmlStr,
    /// The doctype public ID.
    pub public_id: HtmlStr,
    /// The doctype system ID.
    pub system_id: HtmlStr,
}

impl Doctype {
    /// Returns the doctype name.
    pub fn name(&self) -> &str {
        self.name.deref()
    }

    /// Returns the doctype public ID.
    pub fn public_id(&self) -> &str {
        self.public_id.deref()
    }

    /// Returns the doctype system ID.
    pub fn system_id(&self) -> &str {
        self.system_id.deref()
    }
}

impl Debug for Doctype {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<!DOCTYPE {} PUBLIC {:?} {:?}>", self.name(), self.public_id(), self.system_id())
    }
}

/// An HTML element.
#[derive(Clone, PartialEq, Eq)]
pub struct NodeData {
    pub(crate) name: QualName,
    pub(crate) attrs: IndexMap<QualName, HtmlStr>,
    id: OnceCell<Option<HtmlStr>>,
    classes: OnceCell<Vec<LocalName>>,
}

impl NodeData {
    #[doc(hidden)]
    pub fn new(name: QualName, attributes: Vec<Attribute>) -> Self {
        let attrs = attributes.into_iter().map(|a| (a.name, crate::tendril_util::make(a.value))).collect();
        NodeData { attrs, name, id: OnceCell::new(), classes: OnceCell::new() }
    }

    /// Chick if element is the given tag
    ///
    ///
    /// # Examples
    ///
    /// ```
    /// # use htmler::node::NodeData;
    /// ```
    pub fn is_a(&self, tag: &str) -> bool {
        self.name.local.as_ref().eq_ignore_ascii_case(tag)
    }

    /// Returns the element name.
    pub fn name(&self) -> &str {
        self.name.local.deref()
    }

    /// Returns the element ID.
    pub fn id(&self) -> Option<&str> {
        self.id
            .get_or_init(|| self.attrs.iter().find(|(name, _)| name.local.as_ref() == "id").map(|(_, value)| value.clone()))
            .as_deref()
    }

    /// Returns true if element has the class.
    pub fn has_class(&self, class: &str) -> bool {
        self.classes().any(|c| CaseSensitivity::AsciiCaseInsensitive.eq(c.as_bytes(), class.as_bytes()))
    }

    /// Returns an iterator over the element's classes.
    pub fn classes(&self) -> HtmlClasses {
        let classes = self.classes.get_or_init(|| {
            let mut classes: Vec<LocalName> = self
                .attrs
                .iter()
                .filter(|(name, _)| name.local.as_ref() == "class")
                .flat_map(|(_, value)| value.split_whitespace().map(LocalName::from))
                .collect();

            classes.sort_unstable();
            classes.dedup();

            classes
        });

        HtmlClasses { inner: classes.iter() }
    }

    /// Returns the value of an attribute.
    pub fn get_attribute(&self, attr: &str) -> Option<&str> {
        let qualname = QualName::new(None, ns!(), LocalName::from(attr));
        self.attrs.get(&qualname).map(Deref::deref)
    }

    /// Returns true if the element has the attribute.
    pub fn has_attribute(&self, attr: &str) -> bool {
        let qualname = QualName::new(None, ns!(), LocalName::from(attr));
        self.attrs.contains_key(&qualname)
    }

    /// Returns an iterator over the element's attributes.
    pub fn attributes(&self) -> HtmlAttributes {
        HtmlAttributes { inner: self.attrs.iter() }
    }
}

/// Iterator over classes.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct HtmlClasses<'a> {
    inner: SliceIter<'a, LocalName>,
}

impl<'a> Iterator for HtmlClasses<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        self.inner.next().map(Deref::deref)
    }
}

/// An iterator over a node's attributes.
pub type AttributesIter<'a> = indexmap::map::Iter<'a, QualName, HtmlStr>;

/// Iterator over attributes.
#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub struct HtmlAttributes<'a> {
    inner: AttributesIter<'a>,
}

impl<'a> Iterator for HtmlAttributes<'a> {
    type Item = (&'a str, &'a str);

    fn next(&mut self) -> Option<(&'a str, &'a str)> {
        self.inner.next().map(|(k, v)| (k.local.deref(), v.deref()))
    }
}

impl Debug for NodeData {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "<{}", self.name())?;
        for (key, value) in self.attributes() {
            write!(f, " {}={:?}", key, value)?;
        }
        write!(f, ">")
    }
}

/// HTML Processing Instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessingInstruction {
    /// The PI target.
    pub target: HtmlStr,
    /// The PI data.
    pub data: HtmlStr,
}

impl Deref for ProcessingInstruction {
    type Target = str;

    fn deref(&self) -> &str {
        self.data.deref()
    }
}

pub(crate) mod serializable;
