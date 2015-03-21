use std::io::Error;

use html5ever::serialize::{Serialize, Serializer, TraversalScope};

use crate::Element;

impl<'a> Serialize for Element<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S, traversal_scope: TraversalScope) -> Result<(), Error> {
        crate::node::serializable::serialize(self.node, serializer, traversal_scope)
    }
}
