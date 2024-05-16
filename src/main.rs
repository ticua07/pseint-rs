use parser::{postfix_stack_evaluator, shunting_yard};

use crate::{
    interpreter::Interpreter,
    lexer::{find_algorithm, Lexer},
};

mod file;
mod interpreter;
mod lexer;
mod memory;
mod parser;
mod tokens;

fn main() {
    let content = file::open_file("./algorithm.psc");
    let (algo_start, algo_end) = find_algorithm(&content);
    let lines: Vec<&str> = content.lines().collect();

    let lines_of_code = lines[1 + algo_start..algo_end].to_vec();
    let interpreter = Interpreter::new(lines_of_code);
    let _ = interpreter.run();

    // let tokens = Lexer::lex("(5*4+3*2)-1".to_string());
    // let postfix = shunting_yard(tokens);
    // let result = postfix_stack_evaluator(postfix);
    // println!("{:?}", result);
}
