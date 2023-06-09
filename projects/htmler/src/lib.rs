#![feature(once_cell)]
#![doc = include_str!("../readme.md")]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]

#[macro_use]
extern crate html5ever;

pub use crate::{html::Html, node::NodeKind, node_ref::Node, selector::Selector};

pub use selectors::attr::CaseSensitivity;

pub mod error;
pub mod html;
pub mod node;
pub mod node_ref;
pub mod selector;

pub(crate) mod tendril_util {
    use html5ever::tendril;
    /// Atomic equivalent to the default `StrTendril` type.
    pub type HtmlStr = tendril::Tendril<tendril::fmt::UTF8, tendril::Atomic>;

    /// Convert a standard tendril into an atomic one.
    pub fn make(s: tendril::StrTendril) -> HtmlStr {
        s.into_send().into()
    }
}

pub use tendril_util::HtmlStr;
