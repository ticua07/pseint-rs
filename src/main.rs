#![deny(clippy::pedantic)]

use clap::Parser;
use log::{debug, info};
use std::path::PathBuf;

use crate::{
    ast::build_ast,
    interpreter::Interpreter,
    lexer::{find_algorithm, Lexer},
};

mod ast;
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
    pretty_env_logger::init();

    let args = Args::parse();

    let content = file::open(args.path);
    let (algo_start, algo_end) = find_algorithm(&content);
    let lines: Vec<&str> = content.lines().collect();

    let lines_of_code = lines[1 + algo_start..algo_end].to_vec();

    let code = lines_of_code
        .iter()
        .map(|f| Lexer::lex(f))
        .filter(|f| !f.is_empty())
        .collect();

    let mut interpreter = Interpreter::new(&lines_of_code);
    let ast = build_ast(code).unwrap();
    // debug!("{:#?}", ast);
    interpreter.run(ast);

    // let tokens = Lexer::lex("(5*4+3*2)-1".to_string());
    // let postfix = shunting_yard(tokens);
    // let result = postfix_stack_evaluator(postfix);
    // println!("{:?}", result);
}
