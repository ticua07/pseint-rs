use lexer::find_algorithm;

use crate::lexer::Lexer;

mod file;
mod lexer;
mod tokens;

fn main() {
    let content = file::open_file("./algorithm.psc");

    let (algo_start, algo_end) = find_algorithm(&content);
    let code = content[(algo_start + 1)..(algo_end - 1)].to_string();

    // for line in code.split(";") {
    // Lexer::lex(line.to_string());
    // }

    Lexer::lex("Definir algoritmo1 Como Caracter;".to_string());
    Lexer::lex("variable = 'hola' hola".to_string());
    Lexer::lex("1 + 1".to_string());
    Lexer::lex("variable = ''".to_string());
    Lexer::lex("variable = 123".to_string());
    Lexer::lex("variable <- 123".to_string());

    Lexer::lex("2 > 1".to_string());
    Lexer::lex("1 < 2".to_string());
    Lexer::lex("2 >= 2".to_string());
    Lexer::lex("2 <= 2".to_string());
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = main(2, 2);
//         assert_eq!(result, 4);
//     }
// }
