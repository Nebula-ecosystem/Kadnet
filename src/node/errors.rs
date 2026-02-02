use crate::routing::id::IdError;

#[derive(Debug)]
pub enum NodeError {
    IdError(IdError),
}
