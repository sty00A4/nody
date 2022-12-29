use crate::*;

pub const WS: [&str; 4] = [" ", "\r", "\t", "\n"];
pub const SYMBOLS: [&str; 11] = ["(", ")", "[", "]", "{", "}", "@", "#", "$", "\"", "'"];
pub type NodeRef = Box<Node>;
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Int { v: i64, pos: Position }, Float{ v: f64, pos: Position }, Char { v: char, pos: Position },
    Bool { v: bool, pos: Position }, String { v: String, pos: Position },
    Type { v: Type, pos: Position },
    Word { v: String, pos: Position }, Key { v: String, pos: Position },
    Node { head: NodeRef, args: Vec<NodeRef>, pos: Position }, Body { nodes: Vec<Node>, pos: Position },
    Vector { nodes: Vec<Node>, pos: Position },
    Closure { node: NodeRef, pos: Position }, Params { node: NodeRef, pos: Position }
}
impl Node {
    pub fn pos(&self) -> &Position {
        match self {
            Node::Int { v:_, pos }              => pos,
            Node::Float { v:_, pos }            => pos,
            Node::Char { v:_, pos }             => pos,
            Node::Bool { v:_, pos }             => pos,
            Node::String { v:_, pos }           => pos,
            Node::Type { v:_, pos }             => pos,
            Node::Word { v:_, pos }             => pos,
            Node::Key { v:_, pos }              => pos,
            Node::Node { head:_, args:_, pos }  => pos,
            Node::Body { nodes:_, pos }         => pos,
            Node::Vector { nodes:_, pos }       => pos,
            Node::Closure { node:_, pos }       => pos,
            Node::Params { node:_, pos }        => pos
        }
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Int { v, pos:_ }            => write!(f, "{v:?}"),
            Node::Float { v, pos:_ }          => write!(f, "{v:?}"),
            Node::Char { v, pos:_ }           => write!(f, "'{v}"),
            Node::Bool { v, pos:_ }           => write!(f, "{v:?}"),
            Node::String { v, pos:_ }         => write!(f, "{v:?}"),
            Node::Type { v, pos:_ }           => write!(f, "{v:?}"),
            Node::Word { v, pos:_ }           => write!(f, "{v}"),
            Node::Key { v, pos:_ }            => write!(f, "@{v}"),
            Node::Node { head, args, pos:_ }  => write!(f, "({head} {})", args.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Node::Body { nodes, pos:_ }       => write!(f, "{{{}}}", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Node::Vector { nodes, pos:_ }     => write!(f, "[{}]", nodes.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            Node::Closure { node, pos:_ }     => write!(f, "#{node}"),
            Node::Params { node, pos:_ }     => write!(f, "${node}"),
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
            "\"" => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance();
                let mut string = String::new();
                while self.get() != "\"" && self.get() != "" {
                    if self.get_char() == '\\' {
                        self.advance();
                        match self.get_char() {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            _ => string.push(self.get_char())
                        }
                        self.advance();
                    } else {
                        string.push(self.get_char());
                        self.advance();
                    }
                }
                if self.get() == "" { return Err(Error::UnclosedString) }
                self.advance();
                match string.parse::<String>() {
                    Ok(string) => Ok(Node::String { v: string, pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) }),
                    Err(_) => Err(Error::ParseString(string))
                }
            }
            "'" => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance();
                let mut c = String::new();
                while self.get() != "\'" && self.get() != "" {
                    if self.get_char() == '\\' {
                        self.advance();
                        match self.get_char() {
                            'n' => c.push('\n'),
                            't' => c.push('\t'),
                            'r' => c.push('\r'),
                            _ => c.push(self.get_char())
                        }
                        self.advance();
                    } else {
                        c.push(self.get_char());
                        self.advance();
                    }
                }
                if self.get() == "" { return Err(Error::UnclosedChar) }
                self.advance();
                match c.parse::<char>() {
                    Ok(c) => Ok(Node::Char { v: c, pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) }),
                    Err(_) => Err(Error::ParseChar(c))
                }
            }
            _ if self.get_char().is_ascii_digit() => {
                let (start_ln, start_col) = (self.ln, self.col);
                let mut number = String::new();
                while self.get_char().is_ascii_digit() {
                    number.push_str(self.get());
                    self.advance();
                }
                if self.get() == "." {
                    number.push('.');
                    self.advance();
                    while self.get_char().is_ascii_digit() {
                        number.push_str(self.get());
                        self.advance();
                    }
                    match number.parse() {
                        Ok(number) => Ok(Node::Float { v: number, pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) }),
                        Err(_) => Err(Error::ParseFloat(number))
                    }
                } else {
                    match number.parse() {
                        Ok(number) => Ok(Node::Int { v: number, pos: Position::new(start_ln..self.ln, start_col..self.col, &self.path) }),
                        Err(e) => match e.kind() {
                            IntErrorKind::PosOverflow => Err(Error::ParseIntOverflow(number)),
                            IntErrorKind::NegOverflow => Err(Error::ParseIntNegOverflow(number)),
                            _ => Err(Error::ParseInt(number)),
                        }
                    }
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
                    "true"      => Ok(Node::Bool { v: true, pos }),
                    "false"     => Ok(Node::Bool { v: false, pos }),
                    "any"       => Ok(Node::Type { v: Type::Any, pos }),
                    "int"       => Ok(Node::Type { v: Type::Int, pos }),
                    "float"     => Ok(Node::Type { v: Type::Float, pos }),
                    "char"      => Ok(Node::Type { v: Type::Char, pos }),
                    "bool"      => Ok(Node::Type { v: Type::Bool, pos }),
                    "str"       => Ok(Node::Type { v: Type::String, pos }),
                    "key"       => Ok(Node::Type { v: Type::Key, pos }),
                    "closure"   => Ok(Node::Type { v: Type::Closure, pos }),
                    "vec"       => Ok(Node::Type { v: Type::Vector(None), pos }),
                    "obj"       => Ok(Node::Type { v: Type::Object, pos }),
                    "fn"        => Ok(Node::Type { v: Type::Function(vec![], None), pos }),
                    "native-fn" => Ok(Node::Type { v: Type::NativFunction(vec![], None), pos }),
                    "type"      => Ok(Node::Type { v: Type::Type, pos }),
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