use log::error;

use crate::{
    error::{Code, PossibleErrors},
    memory::Memoria,
    tokens::Token,
};

fn precedence(token: &Token) -> u8 {
    match token {
        Token::O => 1,
        Token::Y => 2,
        Token::Comparacion
        | Token::MayorA
        | Token::MayorOIgual
        | Token::MenorA
        | Token::MenorOIgual
        | Token::Diferente
        | Token::Igual => 3,
        Token::Suma | Token::Resta => 4,
        Token::Multiplicacion | Token::Division => 5,
        _ => 0,
    }
}

pub fn shunting_yard(expression: Vec<Token>, memory: &Memoria) -> Result<Vec<Token>, Code> {
    let mut stack: Vec<Token> = Vec::new();
    let mut queue: Vec<Token> = Vec::new();

    for token in expression {
        match token {
            Token::Numero(..) | Token::String(_) | Token::Boolean(_) => queue.push(token),

            Token::Variable(ref var_name) => match memory.get(var_name.clone()) {
                Some(token) => queue.push(token.clone()),
                None => {
                    return Err(Code {
                        error: PossibleErrors::VariableNotFound(var_name.clone()),
                    });
                }
            },

            Token::AbrirParentesis | Token::Comparacion => stack.push(token),
            Token::Suma
            | Token::Resta
            | Token::Multiplicacion
            | Token::Division
            | Token::MayorA
            | Token::MayorOIgual
            | Token::MenorA
            | Token::MenorOIgual
            | Token::Igual
            | Token::Diferente
            | Token::Y
            | Token::O => {
                while let Some(op) = stack.last() {
                    if precedence(op) >= precedence(&token) {
                        queue.push(stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                stack.push(token);
            }

            Token::CerrarParentesis => {
                let mut curr_char = stack.pop().unwrap();

                while curr_char != Token::AbrirParentesis {
                    queue.push(curr_char);
                    curr_char = stack.pop().unwrap();
                }
            }

            _ => {
                error!("token {token:?} shouldn't be here");
                return Err(Code {
                    error: PossibleErrors::InvalidInstruction,
                });
            }
        };
    }

    while !stack.is_empty() {
        queue.push(stack.pop().unwrap());
    }

    Ok(queue)
}

struct CalcNode {
    left: Token,
    right: Token,
    operator: Token,
}

impl CalcNode {
    fn get_string_from_token(token: Token) -> Option<String> {
        if let Token::String(i) = token {
            return Some(i);
        }
        None
    }

    fn get_number_from_token(token: &Token) -> Option<(f32, bool)> {
        if let Token::Numero(i, rounded) = token {
            return Some((*i, *rounded));
        }
        None
    }

    fn calculate_operation(result: f32) -> Token {
        let is_rounded = result.fract() == 0.0;
        Token::Numero(result, is_rounded)
    }

    pub fn calculate(self) -> Option<Token> {
        if !(std::mem::discriminant(&self.left) == std::mem::discriminant(&self.right)) {
            return None;
        }

        match self.left {
            Token::Numero(_, _) => {
                let (left, _) = CalcNode::get_number_from_token(&self.left).unwrap();
                let (right, _) = CalcNode::get_number_from_token(&self.right).unwrap();

                match self.operator {
                    Token::Suma => {
                        let result = left + right;
                        Some(CalcNode::calculate_operation(result))
                    }
                    Token::Resta => {
                        let result = left - right;
                        Some(CalcNode::calculate_operation(result))
                    }
                    Token::Multiplicacion => {
                        let result = left * right;
                        Some(CalcNode::calculate_operation(result))
                    }
                    Token::Division => {
                        let result = left / right;
                        Some(CalcNode::calculate_operation(result))
                    }
                    Token::Comparacion => Some(Token::Boolean((left - right).abs() < 0.1)),
                    Token::MayorA => Some(Token::Boolean(left > right)),
                    Token::MayorOIgual => Some(Token::Boolean(left >= right)),
                    Token::MenorA => Some(Token::Boolean(left < right)),
                    Token::MenorOIgual => Some(Token::Boolean(left <= right)),
                    Token::Diferente => Some(Token::Boolean(left != right)),
                    Token::Igual => Some(Token::Boolean(left == right)),

                    _ => None,
                }
            }
            Token::String(_) => {
                let left = CalcNode::get_string_from_token(self.left).unwrap();
                let right = CalcNode::get_string_from_token(self.right).unwrap();

                match self.operator {
                    Token::Suma => Some(Token::String(left + &right)),
                    Token::Comparacion => Some(Token::Boolean(left == right)),
                    _ => None,
                }
            }
            Token::Boolean(left) => {
                if let Token::Boolean(right) = self.right {
                    match self.operator {
                        Token::Y => Some(Token::Boolean(left && right)),
                        Token::O => Some(Token::Boolean(left || right)),
                        Token::Igual => Some(Token::Boolean(left == right)),
                        Token::Diferente => Some(Token::Boolean(left != right)),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

pub fn postfix_stack_evaluator(tokens: Vec<Token>) -> Option<Token> {
    let mut stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Numero(..) | Token::String(_) | Token::Boolean(_) => stack.push(token),
            operator => {
                let right = stack.pop()?;

                let left = stack.pop().unwrap_or(Token::Numero(0f32, false));

                let node = CalcNode {
                    left,
                    right,
                    operator,
                };
                let result = node.calculate();
                result.as_ref()?;
                stack.push(result?);
            }
        }
    }

    let result = stack.pop()?;
    Some(result)
}

#[cfg(test)]
mod parser_tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn shutting_yard_algo() {
        let expression = "(5*4+3*2)-1";
        let tokens = Lexer::lex(expression);
        let memory = Memoria::new();
        let result = shunting_yard(tokens, &memory).unwrap();

        assert_eq!(
            result,
            vec![
                Token::Numero(5.0, true),
                Token::Numero(4.0, true),
                Token::Multiplicacion,
                Token::Numero(3.0, true),
                Token::Numero(2.0, true),
                Token::Multiplicacion,
                Token::Suma,
                Token::Numero(1.0, true),
                Token::Resta,
            ]
        )
    }

    #[test]
    fn postfix_arithmetic() {
        let expression = "(5*4+3*2)-1";
        let tokens = Lexer::lex(expression);
        let memory = Memoria::new();

        let postfix = shunting_yard(tokens, &memory).unwrap();
        let result = postfix_stack_evaluator(postfix);

        assert_eq!(result, Some(Token::Numero(25.0, true)));
    }

    #[test]
    fn postfix_concatenate() {
        let expression = "'hola' + ' mundo'";
        let tokens = Lexer::lex(expression);
        let memory = Memoria::new();

        let postfix = shunting_yard(tokens, &memory).unwrap();
        let result = postfix_stack_evaluator(postfix);

        assert_eq!(result, Some(Token::String("hola mundo".to_string())));
    }

    #[test]
    fn postfix_boolean() {
        let expression = "VERDADERO Y VERDADERO";
        let tokens = Lexer::lex(expression);
        let memory = Memoria::new();

        let postfix = shunting_yard(tokens, &memory).unwrap();
        let result = postfix_stack_evaluator(postfix);

        assert_eq!(result, Some(Token::Boolean(true)))
    }

    #[test]
    fn postfix_error() {
        let invalid_expressions = vec!["'hola' - 10", "'hola' - 'chau'", "10 - 'hola'"];
        for expr in invalid_expressions {
            let tokens = Lexer::lex(expr);
            let memory = Memoria::new();

            let postfix = shunting_yard(tokens, &memory).unwrap();

            // Should return None when adding 2 different types
            let result = postfix_stack_evaluator(postfix);

            assert_eq!(result, None);
        }
    }
}
