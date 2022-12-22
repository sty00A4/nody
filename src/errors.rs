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
pub enum Error {}