use log::{debug, error, info};

use crate::{
    error::{Code, PossibleErrors},
    tokens::{Keyword, Token, Type},
};

#[derive(Debug, Clone)]
pub enum ASTNode {
    VariableDeclaration {
        names: Vec<String>,
        var_type: Type,
    },
    Assignment {
        name: String,
        expression: Vec<Token>,
    },
    WriteStatement {
        expressions: Vec<Token>,
    },
    ReadStatement {
        variables: Vec<String>,
    },
    IfStatement {
        condition: Vec<Token>,
        code: Vec<Vec<Token>>,
    },
}

fn parse_variable_declaration(tokens: &[Token]) -> Result<(Vec<String>, Type), Code> {
    if tokens.len() < 3 {
        return Err(Code {
            error: PossibleErrors::SyntaxError,
        });
    }

    // último token debe ser Tipo, penúltimo debe ser Instruccion(Como)
    if let (Some(Token::Instruccion(Keyword::Como)), Some(Token::Tipo(t))) =
        (tokens.get(tokens.len() - 2), tokens.get(tokens.len() - 1))
    {
        let vars = tokens[..tokens.len() - 2]
            .iter()
            .filter_map(|t| {
                if let Token::Variable(name) = t {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        Ok((vars, t.clone()))
    } else {
        Err(Code {
            error: PossibleErrors::MissingTypeOrUnvalidType,
        })
    }
}

fn extract_condition(tokens: &[Token]) -> Result<Vec<Token>, Code> {
    if let [Token::AbrirParentesis, middle @ .., Token::CerrarParentesis, Token::Instruccion(Keyword::Entonces)] =
        tokens
    {
        Ok(middle.to_vec())
    } else {
        Err(Code {
            error: PossibleErrors::SyntaxError,
        })
    }
}

pub fn build_ast(code: Vec<Vec<Token>>) -> Result<Vec<ASTNode>, Code> {
    let mut ast = Vec::new();
    let mut i = 0;

    while i < code.len() {
        let line = &code[i];

        match line.as_slice() {
            // Definir a, b, c Como Entero
            [Token::Instruccion(Keyword::Definir), rest @ ..] => {
                let (vars, tipo) = parse_variable_declaration(rest)?;
                ast.push(ASTNode::VariableDeclaration {
                    names: vars,
                    var_type: tipo,
                });
            }

            // nombre <- expresión
            [Token::Variable(var), Token::Igual, rest @ ..] => {
                ast.push(ASTNode::Assignment {
                    name: var.clone(),
                    expression: rest.to_vec(),
                });
            }

            // Escribir a, "hola", b
            [Token::Instruccion(Keyword::Escribir), rest @ ..] => {
                ast.push(ASTNode::WriteStatement {
                    expressions: rest
                        .iter()
                        .map(|f| {
                            if f == &Token::SeparadorArgumento {
                                Token::Suma
                            } else {
                                f.clone()
                            }
                        })
                        .collect(),
                });
            }

            // Leer a, b
            [Token::Instruccion(Keyword::Leer), rest @ ..] => {
                let vars = rest
                    .iter()
                    .filter_map(|t| {
                        if let Token::Variable(name) = t {
                            Some(name.clone())
                        } else {
                            None
                        }
                    })
                    .collect();
                ast.push(ASTNode::ReadStatement { variables: vars });
            }

            // Si (...) Entonces
            [Token::Instruccion(Keyword::Si), rest @ ..] => {
                let condition = extract_condition(rest)?;
                let mut if_code = Vec::new();

                i += 1; // move to the next line after Si

                while i < code.len() {
                    if code[i] == [Token::Instruccion(Keyword::FinSi)] {
                        break;
                    }
                    if_code.push(code[i].clone());
                    i += 1;
                }

                ast.push(ASTNode::IfStatement {
                    condition,
                    code: if_code,
                });

                // skip the FinSi line
                i += 1;
                continue; // skip incrementing i again at the end of the loop
            }

            // FinSi already handled inside the If block
            [Token::Instruccion(Keyword::FinSi)] => {}

            err => {
                error!("Instruction that gave the error: {:?}", err);
                return Err(Code {
                    error: PossibleErrors::SyntaxError,
                });
            }
        }

        i += 1; // only increment if not inside a special block
    }

    Ok(ast)
}
