use super::Bucket;
use crate::Node;

/// What should the main algorithm do with a node?
pub enum Action {
    Process(Node),
    ToLater((Node, Bucket)),
    Nothing,
}
