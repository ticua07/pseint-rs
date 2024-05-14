use crate::tokens::Token;

pub fn shunting_yard(expression: Vec<Token>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::new();
    let mut queue: Vec<Token> = Vec::new();

    for token in expression {
        match token {
            Token::Numero(..) | Token::Float(_) | Token::String(_) => queue.push(token),

            Token::AbrirParentesis => stack.push(token),
            Token::Suma | Token::Resta => {
                // Suma y Resta tienen mas precedencia, entonces se reemplazan por los simbolos
                // de menor precedencia siendo la multiplicación y división

                // Esto se hace para que no ocurra sumas o restas antes de una división o multiplicación
                let last_item = stack.last().unwrap_or(&Token::None);

                if *last_item == Token::Multiplicacion || *last_item == Token::Division {
                    let last_item = stack.pop().unwrap();
                    queue.push(last_item);
                    stack.push(token);
                } else {
                    stack.push(token);
                }
            }

            Token::Multiplicacion | Token::Division => {
                stack.push(token);
            }

            Token::MayorA | Token::MayorOIgual | Token::MenorA | Token::MenorOIgual => {
                stack.push(token)
            }

            Token::Comparacion => stack.push(token),

            Token::CerrarParentesis => {
                let mut curr_char = stack.pop().unwrap();

                while curr_char != Token::AbrirParentesis {
                    queue.push(curr_char);
                    curr_char = stack.pop().unwrap()
                }
            }

            _ => {
                println!("token {:?} shouldn't be here", token)
            }
        };
    }

    while stack.len() != 0 {
        queue.push(stack.pop().unwrap());
    }

    queue
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

    fn get_number_from_token(token: Token) -> Option<(f32, bool)> {
        if let Token::Numero(i, rounded) = token {
            return Some((i, rounded));
        }
        None
    }

    fn calculate_operation(result: f32) -> Token {
        let is_rounded = result.fract() == 0.0;
        return Token::Numero(result, is_rounded);
    }

    pub fn calculate(self) -> Option<Token> {
        if !(std::mem::discriminant(&self.left) == std::mem::discriminant(&self.right)) {
            return None;
        }

        match self.left {
            Token::Numero(_, _) => {
                let (left, _) = CalcNode::get_number_from_token(self.left).unwrap();
                let (right, _) = CalcNode::get_number_from_token(self.right).unwrap();

                match self.operator {
                    Token::Suma => {
                        let result = left + right;
                        return Some(CalcNode::calculate_operation(result));
                    }
                    Token::Resta => {
                        let result = left - right;
                        return Some(CalcNode::calculate_operation(result));
                    }
                    Token::Multiplicacion => {
                        let result = left * right;
                        return Some(CalcNode::calculate_operation(result));
                    }
                    Token::Division => {
                        let result = left / right;
                        return Some(CalcNode::calculate_operation(result));
                    }
                    Token::Comparacion => return Some(Token::Boolean(left == right)),
                    Token::MayorA => return Some(Token::Boolean(left > right)),
                    Token::MayorOIgual => return Some(Token::Boolean(left >= right)),
                    Token::MenorA => return Some(Token::Boolean(left < right)),
                    Token::MenorOIgual => return Some(Token::Boolean(left <= right)),
                    _ => None,
                }
            }
            Token::String(_) => {
                let left = CalcNode::get_string_from_token(self.left).unwrap();
                let right = CalcNode::get_string_from_token(self.right).unwrap();

                match self.operator {
                    Token::Suma => return Some(Token::String(left + &right)),
                    Token::Comparacion => return Some(Token::Boolean(left == right)),
                    _ => None,
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
            Token::Numero(..) | Token::Float(_) | Token::String(_) => stack.push(token),
            operator => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                let node = CalcNode {
                    left,
                    right,
                    operator,
                };
                let result = node.calculate();
                if result.is_none() {
                    return None;
                }
                stack.push(result.unwrap())
            }
        }
    }

    let result = stack.pop().unwrap();
    Some(result)
}

#[cfg(test)]
mod parser_tests {
    use crate::lexer::Lexer;

    use super::*;

    #[test]
    fn shutting_yard_algo() {
        let expression = "(5*4+3*2)-1";
        let tokens = Lexer::lex(expression.to_string());
        let result = shunting_yard(tokens);

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
        let tokens = Lexer::lex(expression.to_string());
        let postfix = shunting_yard(tokens);
        let result = postfix_stack_evaluator(postfix);

        assert_eq!(result, Some(Token::Numero(25.0, true)));
    }

    #[test]
    fn postfix_concatenate() {
        let expression = "'hola' + ' mundo'";
        let tokens = Lexer::lex(expression.to_string());
        let postfix = shunting_yard(tokens);
        let result = postfix_stack_evaluator(postfix);

        assert_eq!(result, Some(Token::String("hola mundo".to_string())));
    }

    #[test]
    fn postfix_error() {
        let invalid_expressions = vec!["'hola' - 10", "'hola' - 'chau'", "10 - 'hola'"];
        for expr in invalid_expressions {
            let tokens = Lexer::lex(expr.to_string());
            let postfix = shunting_yard(tokens);

            // Should return None when adding 2 different types
            let result = postfix_stack_evaluator(postfix);

            assert_eq!(result, None);
        }
    }
}
