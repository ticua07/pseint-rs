use std::io::{self, Write};

use crate::error::{Code, PossibleErrors};
use crate::lexer::Lexer;
use crate::memory::Memoria;
use crate::parser::{postfix_stack_evaluator, shunting_yard};
use crate::tokens::{convert_to_type, Keyword, Token, Type};

pub struct Interpreter {
    code: Vec<Vec<Token>>,
    memory: Memoria,
}

impl Interpreter {
    pub fn new(lines_of_code: &[&str]) -> Interpreter {
        let code = lines_of_code
            .iter()
            .map(|f| Lexer::lex(f))
            .filter(|f| !f.is_empty())
            .collect();

        let memory = Memoria::new();

        Self { code, memory }
    }

    #[allow(clippy::too_many_lines)]
    pub fn run(&mut self) -> Result<(), Code> {
        for instruction in self.code.clone() {
            match instruction.first().unwrap() {
                Token::Instruccion(instr) => match instr {
                    Keyword::Escribir => {
                        let expression = &instruction[1..instruction.len()];
                        let postfix = shunting_yard(expression.to_vec(), &self.memory)?;
                        let result = postfix_stack_evaluator(postfix);
                        if let Some(i) = result {
                            println!("{}", i.get_as_string());
                        } else {
                            return Err(Code {
                                error: PossibleErrors::MissingArguments,
                            });
                        }
                    }
                    Keyword::Leer => {
                        let expression = &instruction[1..instruction.len()];
                        for identifier in expression.iter().cloned() {
                            if let Token::Identificador(var_name) = identifier {
                                let var_type = self.memory.get_type(var_name.clone()).unwrap();

                                print!("> ");
                                io::stdout().flush().unwrap();
                                let mut buffer = String::new();
                                io::stdin().read_line(&mut buffer).unwrap();

                                buffer = buffer.trim().to_string();

                                match var_type {
                                    Type::Caracter => {
                                        self.memory.set(var_name, Token::String(buffer))?;
                                    }
                                    Type::Real => {
                                        let Ok(parsed) = buffer.parse::<f32>() else {
                                            return Err(Code {
                                                error: PossibleErrors::WrongType,
                                            });
                                        };
                                        let value = Token::Numero(parsed, false);

                                        self.memory.set(var_name, value)?;
                                    }
                                    Type::Entero => {
                                        let Ok(parsed) = buffer.parse::<f32>() else {
                                            return Err(Code {
                                                error: PossibleErrors::WrongType,
                                            });
                                        };
                                        if parsed.fract() != 0.0 {
                                            return Err(Code {
                                                error: PossibleErrors::WrongType,
                                            });
                                        }

                                        let value = Token::Numero(parsed, true);

                                        self.memory.set(var_name, value)?;
                                    }
                                    Type::Logico => {
                                        let value = match buffer.to_lowercase().as_str() {
                                            "verdadero" => true,
                                            "falso" => false,
                                            _ => {
                                                return Err(Code {
                                                    error: PossibleErrors::WrongType,
                                                });
                                            }
                                        };
                                        self.memory.set(var_name, Token::Boolean(value))?;
                                    }

                                    Type::None => {
                                        return Err(Code {
                                            error: PossibleErrors::SyntaxError,
                                        })
                                    }
                                }
                            }
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
                            return Err(Code {
                                error: PossibleErrors::MissingTypeOrUnvalidType,
                            });
                        }

                        if identifier_como.is_none() {
                            return Err(Code {
                                error: PossibleErrors::SyntaxError,
                            });
                        }

                        for identifier in expression[0..expression.len().checked_sub(2).unwrap()]
                            .iter()
                            .cloned()
                        {
                            if let Token::Identificador(var_name) = identifier {
                                self.memory
                                    .create(var_name, convert_to_type(var_type).unwrap());
                            }
                        }

                        // dbg!(expression);
                    }
                    _ => {}
                },
                // If it starts with variable, it must be an assignment
                Token::Identificador(var_name) => {
                    let assignment = instruction.get(1);
                    if assignment.is_none() {
                        return Err(Code {
                            error: PossibleErrors::InvalidInstruction,
                        });
                    }
                    if instruction.len() <= 2 {
                        return Err(Code {
                            error: PossibleErrors::IncompleteAssignment,
                        });
                    }

                    let expression = instruction[2..instruction.len()].to_vec();
                    let postfix = shunting_yard(expression, &self.memory)?;
                    let result = postfix_stack_evaluator(postfix);
                    self.memory.set(var_name.clone(), result.unwrap())?;
                }
                _ => {}
            }
        }

        Ok(())
    }
}
