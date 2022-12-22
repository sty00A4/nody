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