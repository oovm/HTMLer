//! Element references.

use ego_tree::{
    iter::{Edge, Traverse},
    NodeRef,
};
use html5ever::serialize::{serialize, SerializeOpts, TraversalScope};

use crate::{node::ElementData, HtmlStr, Node, Selector};

/// Wrapper around a reference to an element node.
///
/// This wrapper implements the `Element` trait from the `selectors` crate, which allows it to be
/// matched against CSS selectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Element<'a> {
    pub(crate) node: NodeRef<'a, Node>,
}

impl<'a> Element<'a> {
    fn new(node: NodeRef<'a, Node>) -> Self {
        Element { node }
    }

    /// Wraps a `NodeRef` only if it references a `Node::Element`.
    pub fn wrap(node: NodeRef<'a, Node>) -> Option<Self> {
        if node.value().is_element() { Some(Element::new(node)) } else { None }
    }

    /// Returns the `Element` referenced by `self`.
    pub fn value(&self) -> &'a ElementData {
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
    pub fn children(&self) -> impl Iterator<Item = Element<'a>> {
        self.node.children().map(Element::new)
    }
    /// Returns the first child element.
    pub fn first_child(&self) -> Option<Element<'a>> {
        self.node.first_child().map(Element::new)
    }
    /// Returns the last child element.
    pub fn last_child(&self) -> Option<Element<'a>> {
        self.node.last_child().map(Element::new)
    }
    /// Returns the parent element.
    pub fn as_node(&self) -> &'a Node {
        self.node.value()
    }
    /// Returns the parent element.
    pub fn as_element(&self) -> Option<&'a ElementData> {
        self.node.value().as_element()
    }
    /// Returns the parent element.
    pub fn as_text(&self) -> Option<&'a HtmlStr> {
        self.node.value().as_text()
    }
    /// Returns the parent element.
    pub fn descendants(&self) -> impl Iterator<Item = Element<'a>> {
        self.node.descendants().map(Element::new)
    }
    /// Returns the parent element.
    pub fn has_class(&self, class: &str) -> bool {
        self.value().has_class(class)
    }

    pub fn has_attribute(&self, name: &str) -> bool {
        self.value().has_attribute(name)
    }

    /// Returns the value of an attribute.
    pub fn get_attribute(&self, name: &str) -> &'a str {
        self.value().get_attribute(name).unwrap_or("")
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
    scope: Element<'a>,
    inner: Traverse<'a, Node>,
    selector: &'b Selector,
}

impl<'a, 'b> Iterator for Select<'a, 'b> {
    type Item = Element<'a>;

    fn next(&mut self) -> Option<Element<'a>> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                if let Some(element) = Element::wrap(node) {
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
    inner: Traverse<'a, Node>,
}

impl<'a> Iterator for Text<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        for edge in &mut self.inner {
            if let Edge::Open(node) = edge {
                if let Node::Text(ref text) = node.value() {
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
