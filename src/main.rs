#![allow(unused)]
mod errors;
mod value;
mod scan;
mod interpret;
use errors::*;
use value::*;
use scan::*;
use interpret::*;
use std::ops::Range;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut args = args.iter();
    args.next();
    match args.next() {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => {
                let node = scan_file(path, text);
            }
            Err(e) => eprintln!("{e}")
        }
        None => {}
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn context() {
        let mut context = Context::new();
        assert_eq!(context.scopes.len(), 1);
        context.push();
        assert_eq!(context.scopes.len(), 2);
        let scope = context.pop();
        assert!(scope.is_some());
        assert_eq!(context.scopes.len(), 1);
    }
}