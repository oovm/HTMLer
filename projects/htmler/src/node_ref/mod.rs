//! Element references.

use crate::{
    node::{Doctype, NodeData, ProcessingInstruction},
    HtmlStr, NodeKind, Selector,
};
use ego_tree::{
    iter::{Edge, Traverse},
    NodeRef,
};
use html5ever::serialize::{serialize, SerializeOpts, TraversalScope};
use std::fmt::{Debug, Formatter};

/// Wrapper around a reference to an element node.
///
/// This wrapper implements the `Element` trait from the `selectors` crate, which allows it to be
/// matched against CSS selectors.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Node<'a> {
    pub(crate) node: NodeRef<'a, NodeKind>,
}

impl<'a> Debug for Node<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ElementHandler").field(&self.node.id()).finish()
    }
}

impl<'a> Node<'a> {
    fn new(node: NodeRef<'a, NodeKind>) -> Self {
        Node { node }
    }

    /// Wraps a `NodeRef` only if it references a `Node::Element`.
    pub fn wrap(node: NodeRef<'a, NodeKind>) -> Option<Self> {
        if node.value().is_element() { Some(Node::new(node)) } else { None }
    }

    /// Returns the `Element` referenced by `self`.
    pub fn value(&self) -> &'a NodeData {
        self.node.value().as_element().unwrap()
    }

    /// Returns an iterator over descendent elements matching a selector.
    pub fn select<'b>(&self, selector: &'b Selector) -> Select<'a, 'b> {
        let mut inner = self.node.traverse();
        inner.next(); // Skip Edge::Open(self).

        Select { scope: *self, inner, selector }
    }

    fn serialize(&self, traversal_scope: TraversalScope) -> String {
        let opts = SerializeOpts {
            scripting_enabled: true, // It's not clear what this does.
            traversal_scope,
            create_missing_parent: false,
        };
        let mut buf = Vec::new();
        serialize(&mut buf, self, opts).unwrap();
        String::from_utf8(buf).unwrap()
    }

    /// Returns the HTML of this element.
    pub fn html(&self) -> String {
        self.serialize(TraversalScope::IncludeNode)
    }

    /// Returns the inner HTML of this element.
    pub fn inner_html(&self) -> String {
        self.serialize(TraversalScope::ChildrenOnly(None))
    }

    /// Returns an iterator over descendent text nodes.
    pub fn text(&self) -> Text<'a> {
        Text { inner: self.node.traverse() }
    }
    /// Returns an iterator over descendent elements.
    pub fn children(&self) -> impl Iterator<Item = Node<'a>> {
        self.node.children().map(Node::new)
    }
    /// Returns the first child element.
    pub fn first_child(&self) -> Option<Node<'a>> {
        self.node.first_child().map(Node::new)
    }
    /// Returns the last child element.
    pub fn last_child(&self) -> Option<Node<'a>> {
        self.node.last_child().map(Node::new)
    }

    /// Returns the parent element.
    pub fn descendants(&self) -> impl Iterator<Item = Node<'a>> {
        self.node.descendants().map(Node::new)
    }
    /// Returns the parent element.
    pub fn had_class(&self, class: &str) -> bool {
        self.value().has_class(class)
    }
    /// Returns the parent element.
    pub fn has_attribute(&self, name: &str) -> bool {
        self.value().has_attribute(name)
    }
    /// Returns the value of an attribute.
    pub fn get_attribute(&self, name: &str) -> &'a str {
        self.value().get_attribute(name).unwrap_or("")
    }
}

impl<'a> Node<'a> {
    /// Checks if the element is of the given type.
    ///
    /// # Examples
    ///
    /// ```
    /// # use htmler::{Node, Html};
    /// let e = Html::parse_fragment("<p></p>").root_element();
    /// assert!(e.is_a("p"));
    /// assert!(!e.is_a("div"));
    /// ```
    pub fn is_a<S: AsRef<str>>(&self, element: S) -> bool {
        self.value().is_a(element.as_ref())
    }
    /// Returns the next sibling element.
    pub fn as_node(&self) -> &'a NodeKind {
        self.node.value()
    }
    /// Returns the parent element.
    pub fn as_element(&self) -> Option<&'a NodeData> {
        self.as_node().as_element()
    }
    /// Returns the parent element.
    pub fn as_doctype(&self) -> Option<&'a Doctype> {
        match self.as_node() {
            NodeKind::Doctype(t) => Some(t),
            _ => None,
        }
    }
    /// Returns the parent element.
    pub fn as_text(&self) -> Option<&'a HtmlStr> {
        match self.as_node() {
            NodeKind::Text(t) => Some(t),
            _ => None,
        }
    }
    /// Returns self as an element.
    pub fn as_processing_instruction(&self) -> Option<&ProcessingInstruction> {
        match self.as_node() {
            NodeKind::ProcessingInstruction(ref pi) => Some(pi),
            _ => None,
        }
    }
    /// Returns the parent element.
    pub fn as_comment(&self) -> Option<&'a HtmlStr> {
        match self.as_node() {
            NodeKind::Comment(t) => Some(t),
            _ => None,
        }
    }
}

// impl<'a> Deref for ElementRef<'a> {
//     type Target = NodeRef<'a, Node>;
//     fn deref(&self) -> &NodeRef<'a, Node> {
//         &self.node
//     }
// }

/// Iterator over descendent elements matching a selector.
#[derive(Debug, Clone)]
pub struct Select<'a, 'b> {
    scope: Node<'a>,
    inner: Traverse<'a, NodeKind>,
    selector: &'b Selector,
}

impl<'a, 'b> Iterator for Select<'a, 'b> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Node<'a>> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                if let Some(element) = Node::wrap(node) {
                    if self.selector.matches_with_scope(&element, Some(self.scope)) {
                        return Some(element);
                    }
                }
            }
        }
        None
    }
}

/// Iterator over descendent text nodes.
#[derive(Debug, Clone)]
pub struct Text<'a> {
    inner: Traverse<'a, NodeKind>,
}

impl<'a> Iterator for Text<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                if let NodeKind::Text(ref text) = node.value() {
                    return Some(&**text);
                }
            }
        }
        None
    }
}

mod element;
mod serializable;

#[cfg(test)]
mod tests {
    use crate::{html::Html, selector::Selector};

    #[test]
    fn test_scope() {
        let html = r"
            <div>
                <b>1</b>
                <span>
                    <span><b>2</b></span>
                    <b>3</b>
                </span>
            </div>
        ";
        let fragment = Html::parse_fragment(html);
        let sel1 = Selector::try_parse("div > span").unwrap();
        let sel2 = Selector::try_parse(":scope > b").unwrap();

        let element1 = fragment.select(&sel1).next().unwrap();
        let element2 = element1.select(&sel2).next().unwrap();
        assert_eq!(element2.inner_html(), "3");
    }
}
