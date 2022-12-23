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
use core::num::IntErrorKind;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut args = args.iter();
    args.next();
    match args.next() {
        Some(path) => match std::fs::read_to_string(path) {
            Ok(text) => match scan_file(path, text) {
                Ok(node) => {
                    let mut context = Context::new();
                    match interpret(&node, &mut context) {
                        Ok((value, ret)) => if let Some(value) = value { println!("{value}") }
                        Err(e) => eprintln!("{e}")
                    }
                }
                Err(e) => println!("{e}")
            }
            Err(e) => eprintln!("{e}")
        }
        None => {
            println!("This is Nody interpreter is written in Rust.");
            println!("USAGE:");
            println!("  nody [file path] - execute file");
            println!("  ...more comming soon...");
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn context() {
        let mut context = Context::new();
        assert_eq!(context.scopes.len(), 1); // first scope exists
        context.push();                      // second scope
        assert_eq!(context.scopes.len(), 2); // 2 scopes
        let scope = context.pop();           // pop second scope
        assert!(scope.is_some());            // is a scope
        assert_eq!(context.scopes.len(), 1); // 1 scope
    }
    #[test]
    fn context_vars() {
        let mut context = Context::new();
        let path = String::from("<test>");
        let pos = Position::new(0..0, 0..0, &path);
        let x = String::from("x");
        context.create_var(x.clone(), Value::Int(1), false, pos.clone());     // x definition in first scope
        assert_eq!(context.get_var(&x), Some(&Value::Int(1)));                // x accessable correctly
        context.push();                                                       // second scope
        assert_eq!(context.get_var(&x), Some(&Value::Int(1)));                // x still accessable
        let y = String::from("y"); 
        context.create_var(y.clone(), Value::Bool(true), false, pos.clone()); // y definition in second scope
        assert_eq!(context.get_var(&y), Some(&Value::Bool(true)));            // y accessable correctly
        assert_eq!(context.get_var(&x), Some(&Value::Int(1)));                // x in first scope still accessable
        context.pop();                                                        // delete second scope
        assert_eq!(context.get_var(&y), None);                                // y deleted
    }
}