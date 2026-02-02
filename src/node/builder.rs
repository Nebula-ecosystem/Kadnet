use super::{Node, NodeError};

pub struct NodeBuilder {
    listenning_port: u16,
    version: usize,
}

impl NodeBuilder {
    pub fn new(listenning_port: u16) -> Self {
        Self {
            listenning_port,
            version: 1,
        }
    }

    pub fn build(&self) -> Result<Node, NodeError> {
        Node::new(self.listenning_port, self.version)
    }
}
