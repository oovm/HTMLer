//! CSS selectors.

use std::{convert::TryFrom, fmt};

use smallvec::SmallVec;

use html5ever::{LocalName, Namespace};
use selectors::{
    matching,
    parser::{SelectorList, SelectorParseErrorKind},
};

use crate::{error::SelectorErrorKind, Node};

/// Wrapper around CSS selectors.
///
/// Represents a "selector group", i.e. a comma-separated list of selectors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Selector {
    /// The CSS selectors.
    selectors: SmallVec<[selectors::parser::Selector<Simple>; 1]>,
}

impl Selector {
    /// Parses a CSS selector group.
    pub fn new(selectors: &str) -> Self {
        Self::try_parse(selectors).expect("Failed to parse selector:`{selectors}`}")
    }

    /// Parses a CSS selector group.
    pub fn try_parse(selectors: &'_ str) -> Result<Self, SelectorErrorKind> {
        let mut parser_input = cssparser::ParserInput::new(selectors);
        let mut parser = cssparser::Parser::new(&mut parser_input);

        SelectorList::parse(&Parser, &mut parser).map(|list| Selector { selectors: list.0 }).map_err(SelectorErrorKind::from)
    }

    /// Returns true if the element matches this selector.
    pub fn matches(&self, element: &Node) -> bool {
        self.matches_with_scope(element, None)
    }

    /// Returns true if the element matches this selector.
    /// The optional `scope` argument is used to specify which element has `:scope` pseudo-class.
    /// When it is `None`, `:scope` will match the root element.
    pub fn matches_with_scope(&self, element: &Node, scope: Option<Node>) -> bool {
        let mut context =
            matching::MatchingContext::new(matching::MatchingMode::Normal, None, None, matching::QuirksMode::NoQuirks);
        context.scope_element = scope.map(|x| selectors::Element::opaque(&x));
        self.selectors.iter().any(|s| matching::matches_selector(s, 0, None, element, &mut context, &mut |_, _| {}))
    }
}

/// An implementation of `Parser` for `selectors`
struct Parser;

impl<'i> selectors::parser::Parser<'i> for Parser {
    type Impl = Simple;
    type Error = SelectorParseErrorKind<'i>;
}

/// A simple implementation of `SelectorImpl` with no pseudo-classes or pseudo-elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Simple;

impl selectors::parser::SelectorImpl for Simple {
    // see: https://github.com/servo/servo/pull/19747#issuecomment-357106065
    type ExtraMatchingData = String;
    type AttrValue = CssString;
    type Identifier = CssLocalName;
    type LocalName = CssLocalName;
    type NamespaceUrl = Namespace;
    type NamespacePrefix = CssLocalName;
    type BorrowedNamespaceUrl = Namespace;

    type BorrowedLocalName = CssLocalName;
    type NonTSPseudoClass = NonTSPseudoClass;

    type PseudoElement = PseudoElement;
}

/// Wraps [`String`] so that it can be used with [`selectors`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CssString(pub String);

impl<'a> From<&'a str> for CssString {
    fn from(val: &'a str) -> Self {
        Self(val.to_owned())
    }
}

impl AsRef<str> for CssString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl cssparser::ToCss for CssString {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        cssparser::serialize_string(&self.0, dest)
    }
}

/// Wraps [`LocalName`] so that it can be used with [`selectors`]
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct CssLocalName(pub LocalName);

impl<'a> From<&'a str> for CssLocalName {
    fn from(val: &'a str) -> Self {
        Self(val.into())
    }
}

impl cssparser::ToCss for CssLocalName {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str(&self.0)
    }
}

/// Non Tree-Structural Pseudo-Class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NonTSPseudoClass {}

impl selectors::parser::NonTSPseudoClass for NonTSPseudoClass {
    type Impl = Simple;

    fn is_active_or_hover(&self) -> bool {
        false
    }

    fn is_user_action_state(&self) -> bool {
        false
    }
}

impl cssparser::ToCss for NonTSPseudoClass {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str("")
    }
}

/// CSS Pseudo-Element
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PseudoElement {}

impl selectors::parser::PseudoElement for PseudoElement {
    type Impl = Simple;
}

impl cssparser::ToCss for PseudoElement {
    fn to_css<W>(&self, dest: &mut W) -> fmt::Result
    where
        W: fmt::Write,
    {
        dest.write_str("")
    }
}

impl<'i> TryFrom<&'i str> for Selector {
    type Error = SelectorErrorKind<'i>;

    fn try_from(s: &'i str) -> Result<Self, Self::Error> {
        Selector::try_parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn selector_conversions() {
        let s = "#testid.testclass";
        let _sel: Selector = s.try_into().unwrap();

        let s = s.to_owned();
        let _sel: Selector = (*s).try_into().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_selector_conversions() {
        let s = "<failing selector>";
        let _sel: Selector = s.try_into().unwrap();
    }
}
