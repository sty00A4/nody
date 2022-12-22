use crate::*;

pub type NodeRef = Box<Node>;
#[derive(Clone, Debug)]
pub enum Node {
    Int(i64), Float(f64), Bool(bool), String(String), Null,
    Word(String), Key(String),
    Node(NodeRef, Vec<NodeRef>), Body(Vec<NodeRef>), Vector(Vec<NodeRef>)
}

pub fn scan_file(path: &String, text: String) -> Result<Node, Error> {
    Ok(Node::Null)
}