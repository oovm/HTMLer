use std::io::Error;

use ego_tree::{iter::Edge, NodeRef};
use html5ever::serialize::{Serializer, TraversalScope};

use crate::NodeKind;

/// Serialize an HTML node using html5ever serializer.
pub(crate) fn serialize<S: Serializer>(
    self_node: NodeRef<NodeKind>,
    serializer: &mut S,
    traversal_scope: TraversalScope,
) -> Result<(), Error> {
    for edge in self_node.traverse() {
        match edge {
            Edge::Open(node) => {
                if node == self_node && traversal_scope == TraversalScope::ChildrenOnly(None) {
                    continue;
                }

                match *node.value() {
                    NodeKind::Doctype(ref doctype) => {
                        serializer.write_doctype(doctype.name())?;
                    }
                    NodeKind::Comment(ref comment) => {
                        serializer.write_comment(comment)?;
                    }
                    NodeKind::Text(ref text) => {
                        serializer.write_text(text)?;
                    }
                    NodeKind::Element(ref elem) => {
                        let attrs = elem.attrs.iter().map(|(k, v)| (k, &v[..]));
                        serializer.start_elem(elem.name.clone(), attrs)?;
                    }
                    _ => (),
                }
            }

            Edge::Close(node) => {
                if node == self_node && traversal_scope == TraversalScope::ChildrenOnly(None) {
                    continue;
                }
                match node.value() {
                    NodeKind::Element(e) => serializer.end_elem(e.name.clone())?,
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
