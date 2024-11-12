#![deny(clippy::pedantic)]

use clap::Parser;
use std::path::PathBuf;

use crate::{interpreter::Interpreter, lexer::find_algorithm};

mod error;
mod file;
mod interpreter;
mod lexer;
mod memory;
mod parser;
mod tokens;

#[derive(Parser, Debug)]
struct Args {
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let content = file::open(args.path);
    let (algo_start, algo_end) = find_algorithm(&content);
    let lines: Vec<&str> = content.lines().collect();

    let lines_of_code = lines[1 + algo_start..algo_end].to_vec();
    let mut interpreter = Interpreter::new(&lines_of_code);
    let result = interpreter.run();
    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
    // let tokens = Lexer::lex("(5*4+3*2)-1".to_string());
    // let postfix = shunting_yard(tokens);
    // let result = postfix_stack_evaluator(postfix);
    // println!("{:?}", result);
}
