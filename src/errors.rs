use crate::*;

pub struct Position { pub ln: Range<usize>, pub col: Range<usize> }
impl Position {
    pub fn new(ln: Range<usize>, col: Range<usize>) -> Self { Self { ln, col } }
    pub fn between(p1: Self, p2: Self) -> Self {
        Self { ln: p1.ln.start..p2.ln.end, col: p1.col.start..p2.col.end }
    }
}

pub enum Error {}