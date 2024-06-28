use crate::Node;

/// What should the main algorithm do with a node?
#[derive(Debug)]
pub enum Action {
    Finish((Vec<Node>, f32)),
    Process(Node),
    ToLater(Node),
    Refresh,
    Rotate,
    Nothing,
}
