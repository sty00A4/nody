#![allow(unused)]
pub mod errors;
pub mod value;
pub mod context;
pub mod scan;
pub mod interpret;
pub mod nody_std;
use errors::*;
use value::*;
use context::*;
use scan::*;
use interpret::*;
use nody_std::*;
use std::slice::Iter;
use std::ops::{Range};
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use core::num::IntErrorKind;
use std::cmp::{min, max};
use std::io::Write;
use std::{thread, fs, env};

const STACK_SIZE: usize = 4 * 1024 * 1024;

pub fn run(path: &String, text: String, std_path: Option<String>) -> Result<(Option<Value>, Return), Error> {
    let mut context = std_context(path.clone(), std_path)?;
    interpret(&scan_file(path, text)?, &mut context)
}
pub fn run_context(path: &String, text: String, context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    interpret(&scan_file(path, text)?, context)
}
pub fn run_file(path: &String, std_path: Option<String>) -> Result<(Option<Value>, Return), Error> {
    match std::fs::read_to_string(path) {
        Ok(text) => run(path, text, std_path),
        Err(_) => Err(Error::TargetFileNotFound(path.clone()))
    }
}
pub fn run_file_context(path: &String, context: &mut Context) -> Result<(Option<Value>, Return), Error> {
    match std::fs::read_to_string(path) {
        Ok(text) => run_context(path, text, context),
        Err(_) => Err(Error::TargetFileNotFound(path.clone()))
    }
}

pub fn nody() {
    let args: Vec<String> = env::args().collect();
    let mut args = args.iter();
    args.next();
    let std_path = if let Ok(path) = env::current_exe() {
        let path = path.display().to_string();
        let path_split = path.split("\\").collect::<Vec<&str>>();
        let mut path: Vec<String> = vec![];
        for x in path_split.get(0..path_split.len()-1).unwrap() {
            path.push(x.to_string());
        }
        path.push("nody_std".to_string());
        Some(path.join("\\"))
    } else { None };
    let mut context = std_context("<STD>".to_string(), std_path.clone()).unwrap_or_else(|e| {
        eprintln!("{e}");
        Context::new("<STD>".to_string(), std_path)
    });
    match args.next() {
        Some(path) => match run_file_context(path, &mut context) {
            Ok((value, ret)) => if let Some(value) = value { println!("{value}") }
            Err(e) => println!("{e}\n{}", print_trace(&context.trace))
        }
        None => {
            println!("This is Nody interpreter is written in Rust.");
            println!("USAGE:");
            println!("  nody [file path] - execute file");
        }
    }
}

fn main() {
    let program = thread::Builder::new()
        .name("main".to_string())
        .stack_size(STACK_SIZE)
        .spawn(nody)
        .unwrap();
    program.join().unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn context() {
        let mut context = Context::new("".to_string(), Some("".to_string()));
        assert_eq!(context.scopes.len(), 1); // first scope exists
        context.push();                      // second scope
        assert_eq!(context.scopes.len(), 2); // 2 scopes
        let scope = context.pop();           // pop second scope
        assert!(scope.is_some());            // is a scope
        assert_eq!(context.scopes.len(), 1); // 1 scope
    }
    #[test]
    fn context_vars() -> Result<(), Error> {
        let mut context = Context::new("".to_string(), Some("".to_string()));
        let path = String::from("<test>");
        let pos = Position::new(0..0, 0..0, &path);
        let x = String::from("x");
        context.create_var(x.clone(), Value::Int(1), false, pos.clone(), false)?;     // x definition in first scope
        assert_eq!(context.get_var(&x), Some(&Value::Int(1)));                        // x accessable correctly
        context.push();                                                               // second scope
        assert_eq!(context.get_var(&x), Some(&Value::Int(1)));                        // x still accessable
        let y = String::from("y"); 
        context.create_var(y.clone(), Value::Bool(true), false, pos.clone(), false)?; // y definition in second scope
        assert_eq!(context.get_var(&y), Some(&Value::Bool(true)));                    // y accessable correctly
        assert_eq!(context.get_var(&x), Some(&Value::Int(1)));                        // x in first scope still accessable
        context.pop();                                                                // delete second scope
        assert_eq!(context.get_var(&y), None);                                        // y deleted
        Ok(())
    }
}