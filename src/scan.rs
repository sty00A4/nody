use crate::*;

pub const WS: [&str; 4] = [" ", "\r", "\t", "\n"];
pub const SYMBOLS: [&str; 9] = ["(", ")", "[", "]", "{", "}", "@", "#", "$"];
pub type NodeRef = Box<Node>;
#[derive(Clone, Debug)]
pub enum Node {
    Int { v: i64, pos: Position }, Float{ v: f64, pos: Position },
    Bool { v: bool, pos: Position }, String { v: String, pos: Position }, Null { pos: Position },
    Type { v: Type, pos: Position },
    Word { v: String, pos: Position }, Key { v: String, pos: Position },
    Node { head: NodeRef, args: Vec<NodeRef>, pos: Position }, Body { nodes: Vec<Node>, pos: Position },
    Vector { nodes: Vec<Node>, pos: Position },
    Closure { node: NodeRef, pos: Position }, Params { node: NodeRef, pos: Position }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Int { v, pos }            => write!(f, "{v:?}"),
            Node::Float { v, pos }          => write!(f, "{v:?}"),
            Node::Bool { v, pos }           => write!(f, "{v:?}"),
            Node::String { v, pos }         => write!(f, "{v:?}"),
            Node::Null { pos }              => write!(f, "null"),
            Node::Type { v, pos }           => write!(f, "{v:?}"),
            Node::Word { v, pos }           => write!(f, "{v}"),
            Node::Key { v, pos }            => write!(f, "@{v}"),
            Node::Node { head, args, pos }  => write!(f, "({head} {})", args.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Node::Body { nodes, pos }       => write!(f, "{{{}}}", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Node::Vector { nodes, pos }     => write!(f, "[{}]", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Node::Closure { node, pos }     => write!(f, "#{node}"),
            Node::Params { node, pos }     => write!(f, "${node}"),
        }
    }
}

pub struct Scanner {
    pub idx: usize, pub ln: usize, pub col: usize,
    pub text: String, pub path: String,
}
impl Scanner {
    pub fn new(path: &String, text: String) -> Self {
        Self { idx: 0, ln: 0, col: 0, text, path: path.clone() }
    }
    pub fn get(&self) -> &str {
        self.text.get(self.idx..self.idx+1).or_else(|| Some("")).unwrap()
    }
    pub fn get_char(&self) -> char {
        self.text.get(self.idx..self.idx+1).or_else(|| Some("")).unwrap().chars().next().or_else(|| Some('\0')).unwrap()
    }
    pub fn advance(&mut self) {
        self.idx += 1; self.col += 1;
        if self.get() == "\n" {
            self.ln += 1;
            self.col = 0;
        }
    }
    pub fn advance_ws(&mut self) {
        while WS.contains(&self.get()) {
            self.advance();
        }
    }
    pub fn scan(&mut self) -> Result<Node, Error> {
        let mut nodes: Vec<Node> = vec![];
        while self.get() != "" {
            let node = self.node()?; self.advance_ws();
            nodes.push(node);
        }
        if nodes.len() == 1 {
            Ok(nodes[0].clone())
        } else {
            Ok(Node::Body { nodes, pos: Position::new(0..self.ln, 0..self.col, &self.path) })
        }
    }
    pub fn node(&mut self) -> Result<Node, Error> {
        match self.get() {
            "(" => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                let head = Box::new(self.node()?); self.advance_ws();
                let mut args: Vec<Box<Node>> = vec![];
                while self.get() != ")" && self.get() != "" {
                    let arg = Box::new(self.node()?); self.advance_ws();
                    args.push(arg);
                }
                self.advance();
                Ok(Node::Node { head, args, pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) })
            }
            "{" => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                let mut nodes: Vec<Node> = vec![];
                while self.get() != "}" && self.get() != "" {
                    let node = self.node()?; self.advance_ws();
                    nodes.push(node);
                }
                self.advance();
                Ok(Node::Body { nodes, pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) })
            }
            "[" => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                let mut nodes: Vec<Node> = vec![];
                while self.get() != "]" && self.get() != "" {
                    let node = self.node()?; self.advance_ws();
                    nodes.push(node);
                }
                self.advance();
                Ok(Node::Vector { nodes, pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) })
            }
            "@" => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance();
                let mut word = String::new();
                while !WS.contains(&self.get()) && !SYMBOLS.contains(&self.get()) && self.get() != "" {
                    word.push_str(self.get());
                    self.advance();
                }
                Ok(Node::Key { v: word, pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) })
            }
            "#" => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                let node = self.node()?;
                Ok(Node::Closure { node: Box::new(node), pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) })
            }
            "$" => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                let node = self.node()?;
                Ok(Node::Params { node: Box::new(node), pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) })
            }
            _ if self.get_char().is_ascii_digit() => {
                let (start_ln, start_col) = (self.ln, self.col);
                let mut number = String::new();
                while self.get_char().is_ascii_digit() {
                    number.push_str(self.get());
                    self.advance();
                }
                if self.get() == "." {
                    self.advance();
                    while self.get_char().is_ascii_digit() {
                        number.push_str(self.get());
                        self.advance();
                    }
                    Ok(Node::Float { v: number.parse().unwrap(), pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) })
                } else {
                    Ok(Node::Int { v: number.parse().unwrap(), pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) })
                }
            }
            _ => {
                let (start_ln, start_col) = (self.ln, self.col);
                let mut word = String::new();
                while !WS.contains(&self.get()) && !SYMBOLS.contains(&self.get()) && self.get() != "" {
                    word.push_str(self.get());
                    self.advance();
                }
                let pos = Position::new(start_ln..self.ln, start_col..self.col, &self.path);
                match word.as_str() {
                    "int"     => Ok(Node::Type { v: Type::Int, pos }),
                    "int8"    => Ok(Node::Type { v: Type::Int8, pos }),
                    "int16"   => Ok(Node::Type { v: Type::Int16, pos }),
                    "int32"   => Ok(Node::Type { v: Type::Int32, pos }),
                    "int64"   => Ok(Node::Type { v: Type::Int64, pos }),
                    "int128"  => Ok(Node::Type { v: Type::Int128, pos }),
                    "uint"    => Ok(Node::Type { v: Type::UInt, pos }),
                    "uint8"   => Ok(Node::Type { v: Type::UInt8, pos }),
                    "uint16"  => Ok(Node::Type { v: Type::UInt16, pos }),
                    "uint32"  => Ok(Node::Type { v: Type::UInt32, pos }),
                    "uint64"  => Ok(Node::Type { v: Type::UInt64, pos }),
                    "uint128" => Ok(Node::Type { v: Type::UInt128, pos }),
                    "char"    => Ok(Node::Type { v: Type::Char, pos }),
                    "bool"    => Ok(Node::Type { v: Type::Bool, pos }),
                    "str"     => Ok(Node::Type { v: Type::String, pos }),
                    "vec"     => Ok(Node::Type { v: Type::Vector(None), pos }),
                    "fn"      => Ok(Node::Type { v: Type::Function(vec![], None), pos }),
                    "type"    => Ok(Node::Type { v: Type::Type, pos }),
                    _ => Ok(Node::Word { v: word, pos })
                }
            }
        }
    }
}

pub fn scan_file(path: &String, text: String) -> Result<Node, Error> {
    let mut scanner = Scanner::new(path, text);
    scanner.scan()
}