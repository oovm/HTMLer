use std::io::Error;

use html5ever::serialize::{Serialize, Serializer, TraversalScope};

use crate::Node;

impl<'a> Serialize for Node<'a> {
    fn serialize<S: Serializer>(&self, serializer: &mut S, traversal_scope: TraversalScope) -> Result<(), Error> {
        crate::node::serializable::serialize(self.ptr, serializer, traversal_scope)
    }
}
