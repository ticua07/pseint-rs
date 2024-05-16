use std::fmt;

use crate::lexer::Lexer;
use crate::memory::Memoria;
use crate::parser::{postfix_stack_evaluator, shunting_yard};
use crate::tokens::{convert_to_type, Keyword, Token, Type};

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

    pub fn run(&mut self) -> Result<(), CodeError> {
        for instruction in self.code.clone() {
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
                    Keyword::Definir => {
                        let expression = instruction[1..instruction.len()].to_vec();
                        let var_type = expression.last().unwrap();
                        let identifier_como =
                            expression.get(expression.len().checked_sub(2).unwrap());

                        // Could probably remove the Type::None, as it is not used anywhere
                        if !(std::mem::discriminant(var_type)
                            == std::mem::discriminant(&Token::Tipo(Type::None)))
                            || *var_type == Token::Tipo(Type::None)
                        {
                            return Err(CodeError {
                                error: PossibleErrors::MissingTypeOrUnvalidType,
                            });
                        }

                        if identifier_como.is_none() {
                            return Err(CodeError {
                                error: PossibleErrors::SyntaxError,
                            });
                        }

                        for identifier in
                            expression[0..expression.len().checked_sub(2).unwrap()].to_vec()
                        {
                            match identifier {
                                Token::Identificador(var_name) => {
                                    println!("DEBUG: setting {} to {}", &var_name, &var_type);
                                    self.memory.create(
                                        var_name,
                                        convert_to_type(var_type.to_owned()).unwrap(),
                                    );
                                }
                                _ => {}
                            }
                        }

                        // dbg!(expression);
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
    MissingTypeOrUnvalidType,
    WrongType,
    SyntaxError,
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
            PossibleErrors::MissingTypeOrUnvalidType => {
                write!(f, "ERROR 46: Falta tipo de dato o tipo no válido.")
            }
            PossibleErrors::SyntaxError => write!(f, "ERROR -1: Error de sintaxis."),
        }
    }
}
