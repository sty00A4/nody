use crate::*;

#[derive(Debug, Clone)]
pub struct Position { pub ln: Range<usize>, pub col: Range<usize>, pub path: String }
impl Position {
    pub fn new(ln: Range<usize>, col: Range<usize>, path: &String) -> Self { Self { ln, col, path: path.clone() } }
    pub fn between(p1: Self, p2: Self) -> Self {
        Self { ln: p1.ln.start..p2.ln.end, col: p1.col.start..p2.col.end, path: p1.path.clone() }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    ParseFloat(String), ParseInt(String), ParseIntOverflow(String), ParseIntNegOverflow(String),
    NotDefined(String), AlreadyDefined(String), Immutable(String),
    Expected, ExpectedType(Type, Type), ExpectedTypes(Vec<Type>, Type),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFloat(n) => write!(f, "ERROR: {n} couldn't be parsed as a float"),
            Self::ParseInt(n) => write!(f, "ERROR: {n} couldn't be parsed as an int"),
            Self::ParseIntOverflow(n) => write!(f, "ERROR: {n} overflowed max int64 value"),
            Self::ParseIntNegOverflow(n) => write!(f, "ERROR: {n} underflowed min int64 value"),
            Self::NotDefined(id) => write!(f, "ERROR: {id:?} is not defined"),
            Self::AlreadyDefined(id) => write!(f, "ERROR: {id:?} is already defined"),
            Self::Immutable(id) => write!(f, "ERROR: {id:?} is immutable"),
            Self::Expected => write!(f, "ERROR: expected a value for the head"),
            Self::ExpectedType(t1, t2) => write!(f, "ERROR: expected {t1}, got {t2}"),
            Self::ExpectedTypes(t, t2) => write!(f, "ERROR: expected {}, got {t2}",
            t.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("|")),
        }
    }
}