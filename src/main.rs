use parser::{postfix_stack_evaluator, shunting_yard};

use crate::lexer::Lexer;

mod file;
mod lexer;
mod parser;
mod tokens;

fn main() {
    // let content = file::open_file("./algorithm.psc");
    // let (algo_start, algo_end) = find_algorithm(&content);
    // let _code = content[(algo_start + 1)..(algo_end - 1)].to_string();

    let tokens = Lexer::lex("(5*4+3*2)-1".to_string());
    let postfix = shunting_yard(tokens);
    let result = postfix_stack_evaluator(postfix);
    println!("{:?}", result);
}
