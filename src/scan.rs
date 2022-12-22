use crate::*;

pub enum Node {
    Int(i64), Float(f64), Bool(bool), String(String), Null,
    Word(String), Key(String),
    Node(Box<Node>, Vec<Node>), Body(Vec<Node>), Vector(Vec<Node>)
}

pub fn scan_file(path: &String, text: String) -> Result<Node, Error> {
    Ok(Node::Null)
}