use crate::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Position { pub ln: Range<usize>, pub col: Range<usize>, pub path: String }
impl Position {
    pub fn new(ln: Range<usize>, col: Range<usize>, path: &String) -> Self { Self { ln, col, path: path.clone() } }
    pub fn between(p1: Self, p2: Self) -> Self {
        Self { ln: p1.ln.start..p2.ln.end, col: p1.col.start..p2.col.end, path: p1.path.clone() }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    TargetFileNotFound(String),
    UnexpectedEnd, ExpectedSymbol(char, char), ExpectedWord,
    ParseFloat(String), ParseInt(String), ParseIntOverflow(String), ParseIntNegOverflow(String),
    ParseChar(String), ParseBool(String), ParseString(String),
    UnclosedChar, UnclosedString,
    NotDefined(String), AlreadyDefined(String), Immutable(String),
    Expected, ExpectedArg, ExpectedType(Type, Type), ExpectedTypes(Vec<Type>, Type),
    FunctionPatternNotFound(String, Vec<Type>),
    InvalidHeadValue(Value), InvalidHeadCastType(Type), InvalidCastBetween(Type, Type),
    IndexOutOfRange(usize, usize)
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TargetFileNotFound(path) => write!(f, "ERROR: {path:?} couldn't be found in the current directory"),
            Self::UnexpectedEnd => write!(f, "ERROR: unexpected end of input"),
            Self::ExpectedSymbol(expected, got) => write!(f, "ERROR: expected {expected:?}, got {got:?}"),
            Self::ExpectedWord => write!(f, "ERROR: expected the beginning of a word here"),
            Self::ParseFloat(n) => write!(f, "ERROR: {n:?} couldn't be parsed as a float"),
            Self::ParseInt(n) => write!(f, "ERROR: {n:?} couldn't be parsed as an int"),
            Self::ParseIntOverflow(n) => write!(f, "ERROR: {n:?} overflowed max int64 value"),
            Self::ParseIntNegOverflow(n) => write!(f, "ERROR: {n:?} underflowed min int64 value"),
            Self::ParseChar(n) => write!(f, "ERROR: {n:?} couldn't be parsed as an char"),
            Self::ParseBool(n) => write!(f, "ERROR: {n:?} couldn't be parsed as an bool"),
            Self::ParseString(n) => write!(f, "ERROR: {n:?} couldn't be parsed as an str"),
            Self::UnclosedChar => write!(f, "ERROR: missing \"'\""),
            Self::UnclosedString => write!(f, "ERROR: missing closing quotes"),
            Self::NotDefined(id) => write!(f, "ERROR: {id:?} is not defined"),
            Self::AlreadyDefined(id) => write!(f, "ERROR: {id:?} is already defined"),
            Self::Immutable(id) => write!(f, "ERROR: {id:?} is immutable"),
            Self::Expected => write!(f, "ERROR: expected a value for the head"),
            Self::ExpectedArg => write!(f, "ERROR: expected a value for argument"),
            Self::ExpectedType(t1, t2) => write!(f, "ERROR: expected {t1}, got {t2}"),
            Self::ExpectedTypes(t, t2) => write!(f, "ERROR: expected {}, got {t2}",
            t.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|")),
            Self::FunctionPatternNotFound(id, types) => write!(f, "ERROR: no function {id:?} found with pattern ({})",
            types.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")),
            Self::InvalidHeadValue(v) => write!(f, "ERROR: unexpected {} value for head", v.typ()),
            Self::InvalidHeadCastType(t) => write!(f, "ERROR: invalid cast type {t}"),
            Self::InvalidCastBetween(t1, t2) => write!(f, "ERROR: invalid cast from {t2} to {t1}"),
            Self::IndexOutOfRange(idx, size) => write!(f, "ERROR: index {idx} out of range of size {size}"),
        }
    }
}

pub fn print_trace(trace: &Vec<(Position)>) -> String {
    let mut string = String::new();
    for pos in trace.iter() {
        string.push_str("in ");
        string.push_str(pos.path.as_str());
        string.push(':');
        string.push_str(pos.ln.start.to_string().as_str());
        string.push(':');
        string.push_str(pos.col.start.to_string().as_str());
        string.push('\n');
        let text = match std::fs::read_to_string(&pos.path) {
            Ok(text) => text,
            Err(_) => String::from("FILE NOT FOUND")
        };
        let text_lines: Vec<&str> = text.split("\n").collect();
        let lines = text_lines.get(pos.ln.clone()).unwrap_or_else(|| &["LINES OUT OF RANGE"]);
        for line in lines { string.push_str(line) }
    }
    string
}