use std::io::{self, Write};

use log::{debug, info, trace, warn};

use crate::ast::{build_ast, ASTNode};
use crate::error::{Code, PossibleErrors};
use crate::lexer::Lexer;
use crate::memory::Memoria;
use crate::parser::{postfix_stack_evaluator, shunting_yard};
use crate::tokens::{convert_to_type, Keyword, Token, Type};

pub struct Interpreter {
    code: Vec<Vec<Token>>,
    memory: Memoria,
}

fn check_same_type(t1: &Token, t2: &Token) -> bool {
    std::mem::discriminant(t1) == std::mem::discriminant(t2)
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

    pub fn run(&mut self, ast: Vec<ASTNode>) {
        for statement in ast {
            match statement {
                ASTNode::VariableDeclaration { names, var_type } => {
                    for name in names {
                        trace!("Create {name}, set to {var_type}");
                        self.memory.create(name, var_type);
                    }
                }
                ASTNode::Assignment { name, expression } => {
                    // expression is a vec of tokens, if the length is more than 1 then it's
                    // probably a mathematical expression, otherwise it's just 1 token
                    // like falso for example, which just set the variable to false in one token

                    // it technically is more expensive to retrieve this value first
                    // then check for shunting_yard, but I can refactor it later!
                    let mut result: Token = expression.get(0).unwrap().clone();

                    if let Ok(postfix) = shunting_yard(expression, &self.memory) {
                        // result = postfix_stack_evaluator().unwrap();
                        result = postfix_stack_evaluator(postfix).unwrap();
                    } else {
                        warn!("shunting_yard couldn't be completed")
                    }

                    trace!("Set {} to {}", name, result);
                    self.memory.set(name, result).unwrap();

                    // debug!("{:?}", self.memory);
                }
                ASTNode::WriteStatement { expressions } => {
                    let mut result: Token = expressions.get(0).unwrap().clone();

                    if let Ok(postfix) = shunting_yard(expressions, &self.memory) {
                        result = postfix_stack_evaluator(postfix).unwrap();
                    } else {
                        warn!("shunting_yard couldn't be completed")
                    }

                    println!("{}", result.get_as_string());
                }
                ASTNode::IfStatement { condition, code } => {
                    trace!("{:?}, {:?}", condition, code);

                    let mut result: Token = Token::None;

                    if condition.len() == 1 {
                        result = condition.get(0).unwrap().clone();
                    }

                    if let Ok(postfix) = shunting_yard(condition, &self.memory) {
                        if let Some(res) = postfix_stack_evaluator(postfix) {
                            result = res;
                        } else {
                            warn!("postfix couldn't be completed")
                        }
                    } else {
                        warn!("shunting_yard couldn't be completed")
                    }

                    // token is variable
                    // can only be logico type
                    let conditional = match result {
                        Token::Variable(variable) => {
                            if let Some(value) = self.memory.get(variable) {
                                match value.clone().get_as_string().as_str() {
                                    "true" => true,
                                    "false" => false,
                                    _ => false,
                                }
                            } else {
                                false // variable not found
                            }
                        }
                        Token::Boolean(b) => b,
                        _ => false,
                    };

                    debug!("{}", conditional);

                    if conditional {
                        let if_ast = build_ast(code).unwrap();
                        self.run(if_ast);
                    }
                }
                _ => {
                    warn!("unhandled statement")
                }
            }
        }
    }
}
