use std::fmt;

use crate::lexer::Lexer;
use crate::memory::Memoria;
use crate::parser::{postfix_stack_evaluator, shunting_yard};
use crate::tokens::{Keyword, Token};

pub struct Interpreter {
    code: Vec<Vec<Token>>,
    memory: Memoria,
}

impl Interpreter {
    pub fn new(lines_of_code: Vec<&str>) -> Interpreter {
        let code = lines_of_code
            .iter()
            .map(|f| Lexer::lex(f.to_string()))
            .filter(|f| !f.is_empty())
            .collect();

        let memory = Memoria::new();

        return Self { code, memory };
    }

    pub fn run(self) -> Result<(), CodeError> {
        for instruction in self.code {
            match instruction.first().unwrap() {
                Token::Instruccion(instr) => match instr {
                    Keyword::Escribir => {
                        let expression = &instruction[1..instruction.len()];
                        let postfix = shunting_yard(expression.to_vec());
                        let result = postfix_stack_evaluator(postfix);
                        if let Some(i) = result {
                            println!("{}", i.get_as_string());
                        } else {
                            return Err(CodeError {
                                error: PossibleErrors::MissingArguments,
                            });
                        }
                    }
                    Keyword::None | _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
enum PossibleErrors {
    MissingArguments,
    WrongType,
}

#[derive(Debug, Clone)]
pub struct CodeError {
    error: PossibleErrors,
}

impl fmt::Display for CodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error {
            PossibleErrors::MissingArguments => write!(f, "ERROR 53: Faltan parámetros."),
            PossibleErrors::WrongType => write!(f, "ERROR 125: No coinciden los tipos."),
        }
    }
}
