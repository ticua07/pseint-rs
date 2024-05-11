use crate::tokens::Token;

pub fn shunting_yard(expression: Vec<Token>) -> Vec<Token> {
    let mut stack: Vec<Token> = Vec::new();
    let mut queue: Vec<Token> = Vec::new();

    for token in expression {
        match token {
            Token::Numero(_) => queue.push(token),
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
    left: i32,
    right: i32,
    operator: Token,
}

impl CalcNode {
    pub fn calculate(self) -> Option<i32> {
        match self.operator {
            Token::Suma => return Some(self.left + self.right),
            Token::Resta => return Some(self.left - self.right),
            Token::Multiplicacion => return Some(self.left * self.right),
            Token::Division => return Some(self.left / self.right),
            _ => None,
        }
    }
}

pub fn postfix_stack_evaluator(tokens: Vec<Token>) -> i32 {
    let mut stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Numero(_) => stack.push(token),
            operator => {
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                let node = CalcNode {
                    left: get_number_from_token(left).expect("Should be a number"),
                    right: get_number_from_token(right).expect("Should be a number"),
                    operator,
                };
                let result = node.calculate();
                stack.push(Token::Numero(result.expect(
                    "Operator should be Suma, Resta, Multiplicación or División",
                )));
            }
        }
    }

    let result = stack.pop().unwrap();
    dbg!(&result);

    get_number_from_token(result).expect("Invalid Expression")
}

fn get_number_from_token(token: Token) -> Option<i32> {
    if let Token::Numero(i) = token {
        return Some(i);
    }
    None
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
                Token::Numero(5),
                Token::Numero(4),
                Token::Multiplicacion,
                Token::Numero(3),
                Token::Numero(2),
                Token::Multiplicacion,
                Token::Suma,
                Token::Numero(1),
                Token::Resta,
            ]
        )
    }

    #[test]
    fn postfix_result() {
        let expression = "(5*4+3*2)-1";
        let tokens = Lexer::lex(expression.to_string());
        let postfix = shunting_yard(tokens);
        let result = postfix_stack_evaluator(postfix);

        assert_eq!(result, 25);
    }
}
