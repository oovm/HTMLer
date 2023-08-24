use crate::Node;
use html5ever::Namespace;
use selectors::{
    attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint},
    context::MatchingContext,
    matching::ElementSelectorFlags,
    OpaqueElement, SelectorImpl,
};

use crate::selector::{CssLocalName, CssString, PseudoElement, Simple};

/// Note: will never match against non-tree-structure pseudo-classes.
impl<'a> selectors::Element for Node<'a> {
    type Impl = Simple;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self.ptr.value())
    }

    fn parent_element(&self) -> Option<Self> {
        self.ptr.parent().and_then(Node::wrap)
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        self.ptr.prev_siblings().find(|sibling| sibling.value().is_element()).map(Node::new)
    }

    fn next_sibling_element(&self) -> Option<Self> {
        self.ptr.next_siblings().find(|sibling| sibling.value().is_element()).map(Node::new)
    }

    fn first_element_child(&self) -> Option<Self> {
        for child in self.children() {
            if child.ptr.value().is_element() {
                return Some(child);
            }
        }
        None
    }

    fn is_html_element_in_html_document(&self) -> bool {
        // FIXME: Is there more to this?
        self.as_element().unwrap().name.ns == ns!(html)
    }

    fn has_local_name(&self, name: &CssLocalName) -> bool {
        match self.as_element() {
            Some(data) => data.name.local == name.0,
            None => false,
        }
    }

    fn has_namespace(&self, namespace: &Namespace) -> bool {
        match self.as_element() {
            Some(data) => &data.name.ns == namespace,
            None => false,
        }
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.as_element().unwrap().name == other.as_element().unwrap().name
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&Namespace>,
        local_name: &CssLocalName,
        operation: &AttrSelectorOperation<&CssString>,
    ) -> bool {
        self.as_element().unwrap().attrs.iter().any(|(key, value)| {
            !matches!(*ns, NamespaceConstraint::Specific(url) if *url != key.ns)
                && local_name.0 == key.local
                && operation.eval_str(value)
        })
    }

    fn match_non_ts_pseudo_class(
        &self,
        _: &<Self::Impl as SelectorImpl>::NonTSPseudoClass,
        _: &mut MatchingContext<Self::Impl>,
    ) -> bool {
        false
    }

    fn match_pseudo_element(&self, _: &PseudoElement, _context: &mut MatchingContext<Self::Impl>) -> bool {
        true
    }

    fn apply_selector_flags(&self, _: ElementSelectorFlags) {}

    fn is_link(&self) -> bool {
        self.is_a("link")
    }

    fn is_html_slot_element(&self) -> bool {
        true
    }

    fn has_id(&self, id: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        match self.as_element().unwrap().id() {
            Some(val) => case_sensitivity.eq(id.0.as_bytes(), val.as_bytes()),
            None => false,
        }
    }

    fn has_class(&self, name: &CssLocalName, case_sensitivity: CaseSensitivity) -> bool {
        self.as_element().unwrap().classes().any(|c| case_sensitivity.eq(c.as_bytes(), name.0.as_bytes()))
    }

    fn imported_part(&self, _: &CssLocalName) -> Option<CssLocalName> {
        None
    }

    fn is_part(&self, _name: &CssLocalName) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        !self.children().any(|child| child.ptr.value().is_element() || child.ptr.value().is_text())
    }

    fn is_root(&self) -> bool {
        self.ptr.parent().map_or(false, |parent| parent.value().is_document())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        html::Html,
        selector::{CssLocalName, Selector},
        Node,
    };
    use selectors::{attr::CaseSensitivity, Element};

    #[test]
    fn test_has_id() {
        let html = "<p id='link_id_456'>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::try_parse("p").unwrap();

        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(true, element.has_id(&CssLocalName::from("link_id_456"), CaseSensitivity::CaseSensitive));

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(false, element.has_id(&CssLocalName::from("any_link_id"), CaseSensitivity::CaseSensitive));
    }

    #[test]
    fn test_is_link() {
        let html = "<link href='https://www.example.com'>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::try_parse("link").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(true, element.is_link());

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::try_parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(false, element.is_link());
    }

    #[test]
    fn test_has_class() {
        let html = "<p class='my_class'>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::try_parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            true,
            <Node as Element>::has_class(&element, &CssLocalName::from("my_class"), CaseSensitivity::CaseSensitive)
        );

        let html = "<p>hey there</p>";
        let fragment = Html::parse_fragment(html);
        let sel = Selector::try_parse("p").unwrap();
        let element = fragment.select(&sel).next().unwrap();
        assert_eq!(
            false,
            <Node as Element>::has_class(&element, &CssLocalName::from("my_class"), CaseSensitivity::CaseSensitive)
        );
    }
}
