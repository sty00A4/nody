use crate::*;

pub const WS: [char; 4] = [' ', '\r', '\t', '\n'];
pub const SYMBOLS: [char; 12] = [';', '(', ')', '[', ']', '{', '}', '@', '#', '$', '"', '\''];
pub type NodeRef = Box<Node>;
#[derive(Clone, PartialEq)]
pub enum Node {
    None { pos: Position },
    Int { v: i64, pos: Position }, Float{ v: f64, pos: Position }, Char { v: char, pos: Position },
    Bool { v: bool, pos: Position }, String { v: String, pos: Position },
    Type { v: Type, pos: Position },
    Word { v: String, pos: Position }, Key { v: String, pos: Position },
    Node { head: NodeRef, args: Vec<NodeRef>, pos: Position }, Body { nodes: Vec<Node>, pos: Position },
    Vector { nodes: Vec<Node>, pos: Position },
    Closure { node: NodeRef, pos: Position }, Params { params: Vec<(String, NodeRef, bool)>, pos: Position },
    Object { entries: Vec<(String, NodeRef)>, pos: Position }
}
impl Node {
    pub fn pos(&self) -> &Position {
        match self {
            Node::None { pos }                  => pos,
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
            Node::Params { params:_, pos }      => pos,
            Node::Object { entries:_, pos }         => pos,
        }
    }
}
impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::None { pos:_ }              => write!(f, "()"),
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
            Node::Params { params, pos:_ }    => write!(f, "$({})",
            params.iter().map(|(id, typ, more)|
                format!("{} {}{}", id, typ, if *more { "*" } else { "" }))
            .collect::<Vec<String>>().join(" ")),
            Node::Object { entries, pos:_ }    => write!(f, "${{{}}}",
            entries.iter().map(|(key, value)|
                format!("{} {:?}", key, value))
            .collect::<Vec<String>>().join(" ")),
        }
    }
}
impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub fn word_to_node(word: String, pos: Position) -> Node {
    match word.as_str() {
        "true"      => Node::Bool { v: true, pos },
        "false"     => Node::Bool { v: false, pos },
        "any"       => Node::Type { v: Type::Any, pos },
        "int"       => Node::Type { v: Type::Int, pos },
        "float"     => Node::Type { v: Type::Float, pos },
        "char"      => Node::Type { v: Type::Char, pos },
        "bool"      => Node::Type { v: Type::Bool, pos },
        "str"       => Node::Type { v: Type::String, pos },
        "key"       => Node::Type { v: Type::Key, pos },
        "path"      => Node::Type { v: Type::Path, pos },
        "index"     => Node::Type { v: Type::Index, pos },
        "closure"   => Node::Type { v: Type::Closure, pos },
        "vec"       => Node::Type { v: Type::Vector(None), pos },
        "obj"       => Node::Type { v: Type::Object, pos },
        "fn"        => Node::Type { v: Type::Function(vec![], None), pos },
        "native-fn" => Node::Type { v: Type::NativFunction(vec![], None), pos },
        "type"      => Node::Type { v: Type::Type, pos },
        _ => Node::Word { v: word, pos }
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
    pub fn get(&self) -> char {
        self.text.get(self.idx..self.idx+1).or_else(|| Some("")).unwrap().chars().next().or_else(|| Some('\0')).unwrap()
    }
    pub fn advance(&mut self) {
        self.idx += 1; self.col += 1;
        if self.get() == '\n' {
            self.ln += 1;
            self.col = 0;
        }
    }
    pub fn advance_ws(&mut self) {
        while WS.contains(&self.get()) || self.get() == ';' {
            if self.get() == ';' {
                while self.get() == ';' && self.get() != '\0' {
                    while self.get() != '\n' && self.get() != '\0' { self.advance(); }
                }
            } else {
                self.advance();
            }
        }
    }
    pub fn scan(&mut self) -> Result<Node, Error> {
        let mut nodes: Vec<Node> = vec![];
        while self.get() != '\0' {
            let node = self.node()?; self.advance_ws();
            if let Some(node) = node { nodes.push(node); }
        }
        if nodes.len() == 1 {
            Ok(nodes[0].clone())
        } else {
            Ok(Node::Body { nodes, pos: Position::new(0..self.ln+1, 0..self.col, &self.path) })
        }
    }
    pub fn node(&mut self) -> Result<Option<Node>, Error> {
        if self.get() == '\0' { return Ok(None) }
        self.advance_ws();
        match self.get() {
            ')' | ']' | '}' => Err(Error::UnexpectedSymbol(self.get())),
            '(' => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                if self.get() == ')' {
                    self.advance();
                    let (stop_ln, stop_col) = (self.ln, self.col);
                    self.advance_ws();
                    return Ok(Some(Node::None { pos: Position::new(start_ln..stop_ln, start_col..stop_col, &self.path) }))
                }
                let head = self.node()?; self.advance_ws();
                if head.is_none() { return Err(Error::UnexpectedEnd) }
                let head = Box::new(head.unwrap());
                let mut args: Vec<Box<Node>> = vec![];
                while self.get() != ')' && self.get() != '\0' {
                    let arg = self.node()?; self.advance_ws();
                    if arg.is_none() { return Err(Error::UnexpectedEnd) }
                    let arg = Box::new(arg.unwrap());
                    args.push(arg);
                }
                if self.get() == '\0' { return Err(Error::UnexpectedEnd) }
                self.advance();
                Ok(Some(Node::Node { head, args, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) }))
            }
            '{' => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                let mut nodes: Vec<Node> = vec![];
                while self.get() != '}' && self.get() != '\0' {
                    let node = self.node()?; self.advance_ws();
                    if let Some(node) = node { nodes.push(node); }
                }
                if self.get() == '\0' { return Err(Error::UnexpectedEnd) }
                self.advance();
                if nodes.len() == 1 {
                    Ok(Some(nodes[0].clone()))
                } else {
                    Ok(Some(Node::Body { nodes, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) }))
                }
            }
            '[' => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                let mut nodes: Vec<Node> = vec![];
                while self.get() != ']' && self.get() != '\0' {
                    let node = self.node()?; self.advance_ws();
                    if node.is_none() { return Err(Error::UnexpectedEnd) }
                    let node = node.unwrap();
                    nodes.push(node);
                }
                if self.get() == '\0' { return Err(Error::UnexpectedEnd) }
                self.advance();
                Ok(Some(Node::Vector { nodes, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) }))
            }
            '@' => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance();
                let mut word = String::new();
                while !WS.contains(&self.get()) && !SYMBOLS.contains(&self.get()) && self.get() != '\0' {
                    word.push(self.get());
                    self.advance();
                }
                if self.get() == '\0' { return Err(Error::UnexpectedEnd) }
                Ok(Some(Node::Key { v: word, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) }))
            }
            '#' => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                let node = self.node()?;
                if node.is_none() { return Err(Error::UnexpectedEnd) }
                let node = Box::new(node.unwrap());
                Ok(Some(Node::Closure { node, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) }))
            }
            '$' => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance(); self.advance_ws();
                match self.get() {
                    '(' => {
                        self.advance(); self.advance_ws();
                        let mut params: Vec<(String, Box<Node>, bool)> = vec![];
                        while self.get() != ')' && self.get() != '\0' {
                            let mut param = String::new();
                            while !WS.contains(&self.get()) && !SYMBOLS.contains(&self.get()) && self.get() != '\0' {
                                param.push(self.get());
                                self.advance();
                            }
                            if param.len() == 0 { return Err(Error::ExpectedWord) }
                            self.advance_ws();
                            if SYMBOLS.contains(&self.get()) {
                                let typ = self.node()?;
                                if typ.is_none() { return Err(Error::UnexpectedEnd) }
                                let typ = Box::new(typ.unwrap());
                                let mut more = self.get() == '*';
                                if more { self.advance(); self.advance_ws(); }
                                params.push((param, typ, more));
                            } else {
                                let mut typ = String::new();
                                while !WS.contains(&self.get()) && !SYMBOLS.contains(&self.get()) && self.get() != '*' && self.get() != '\0' {
                                    typ.push(self.get());
                                    self.advance();
                                }
                                let pos = Position::new(start_ln..self.ln+1, start_col..self.col, &self.path);
                                if typ.len() == 0 { return Err(Error::ExpectedWord) }
                                self.advance_ws();
                                let typ = Box::new(word_to_node(typ, pos));
                                let mut more = self.get() == '*';
                                if more { self.advance(); self.advance_ws(); }
                                params.push((param, typ, more));
                            }
                        }
                        if self.get() == '\0' { return Err(Error::UnexpectedEnd) }
                        self.advance();
                        Ok(Some(Node::Params { params, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) }))
                    }
                    '{' => {
                        self.advance(); self.advance_ws();
                        let mut entries: Vec<(String, Box<Node>)> = vec![];
                        while self.get() != '}' && self.get() != '\0' {
                            let mut key = String::new();
                            while !WS.contains(&self.get()) && !SYMBOLS.contains(&self.get()) && self.get() != '\0' {
                                key.push(self.get());
                                self.advance();
                            }
                            if key.len() == 0 { return Err(Error::ExpectedWord) }
                            self.advance_ws();
                            let value = self.node()?; self.advance_ws();
                            if value.is_none() { return Err(Error::UnexpectedEnd) }
                            let value = Box::new(value.unwrap());
                            entries.push((key, value));
                        }
                        if self.get() == '\0' { return Err(Error::UnexpectedEnd) }
                        self.advance();
                        Ok(Some(Node::Object { entries, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) }))
                    }
                    _ => Err(Error::ExpectedSymbols(vec!['(', '{'], self.get()))
                }
            }
            '"' => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance();
                let mut string = String::new();
                while self.get() != '"' && self.get() != '\0' {
                    if self.get() == '\\' {
                        self.advance();
                        match self.get() {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            _ => string.push(self.get())
                        }
                        self.advance();
                    } else {
                        string.push(self.get());
                        self.advance();
                    }
                }
                if self.get() == '\0' { return Err(Error::UnclosedString) }
                self.advance();
                match string.parse::<String>() {
                    Ok(string) => Ok(Some(Node::String { v: string, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) })),
                    Err(_) => Err(Error::ParseString(string))
                }
            }
            '\'' => {
                let (start_ln, start_col) = (self.ln, self.col);
                self.advance();
                let mut c = String::new();
                while self.get() != '\'' && self.get() != '\0' {
                    if self.get() == '\\' {
                        self.advance();
                        match self.get() {
                            'n' => c.push('\n'),
                            't' => c.push('\t'),
                            'r' => c.push('\r'),
                            _ => c.push(self.get())
                        }
                        self.advance();
                    } else {
                        c.push(self.get());
                        self.advance();
                    }
                }
                if self.get() == '\0' { return Err(Error::UnclosedChar) }
                self.advance();
                match c.parse::<char>() {
                    Ok(c) => Ok(Some(Node::Char { v: c, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) })),
                    Err(_) => Err(Error::ParseChar(c))
                }
            }
            // numbers
            _ if self.get().is_ascii_digit() => {
                let (start_ln, start_col) = (self.ln, self.col);
                let mut number = String::new();
                while self.get().is_ascii_digit() {
                    number.push(self.get());
                    self.advance();
                }
                if self.get() == '.' {
                    number.push('.');
                    self.advance();
                    while self.get().is_ascii_digit() {
                        number.push(self.get());
                        self.advance();
                    }
                    match number.parse() {
                        Ok(number) => Ok(Some(Node::Float { v: number, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) })),
                        Err(_) => Err(Error::ParseFloat(number))
                    }
                } else {
                    match number.parse() {
                        Ok(number) => Ok(Some(Node::Int { v: number, pos: Position::new(start_ln..self.ln+1, start_col..self.col, &self.path) })),
                        Err(e) => match e.kind() {
                            IntErrorKind::PosOverflow => Err(Error::ParseIntOverflow(number)),
                            IntErrorKind::NegOverflow => Err(Error::ParseIntNegOverflow(number)),
                            _ => Err(Error::ParseInt(number)),
                        }
                    }
                }
            }
            // words
            _ => {
                let (start_ln, start_col) = (self.ln, self.col);
                let mut word = String::new();
                while !WS.contains(&self.get()) && !SYMBOLS.contains(&self.get()) && self.get() != '\0' {
                    word.push(self.get());
                    self.advance();
                }
                let pos = Position::new(start_ln..self.ln+1, start_col..self.col, &self.path);
                Ok(Some(word_to_node(word, pos)))
            }
        }
    }
}

pub fn scan_file(path: &String, text: String) -> Result<Node, Error> {
    let mut scanner = Scanner::new(path, text);
    scanner.scan()
}