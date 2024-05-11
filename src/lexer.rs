use std::{iter::Peekable, str::Chars};

use crate::tokens::{convert_to_keyword, Token};

pub struct Lexer {}

impl Lexer {
    fn parse_numeric(initial_char: char, chars: &mut Peekable<Chars>) -> Token {
        let mut curr_char = initial_char;
        let mut string = String::new();

        while curr_char.is_numeric() {
            string.push(curr_char);

            // No more characters, this mean the line of code has reached the end.
            if chars.peek().is_none() || !chars.peek().unwrap().is_numeric() {
                break;
            };

            curr_char = chars.next().unwrap();
        }

        let token = Token::Numero(string.parse().unwrap());

        return token;
    }

    fn parse_alphanumeric(initial_char: char, chars: &mut Peekable<Chars>) -> Token {
        let mut curr_char = initial_char;
        let mut string = String::new();

        while curr_char.is_alphanumeric() {
            string.push(curr_char);

            // No more characters, this mean the line of code has reached the end.
            if chars.peek().is_none() {
                break;
            };

            curr_char = chars.next().unwrap();
        }

        convert_to_keyword(string)
    }

    fn parse_string(quote: char, chars: &mut Peekable<Chars>) -> Token {
        // skips first quote
        let mut curr_char = chars.next().unwrap();
        let mut string = String::new();

        while curr_char != quote {
            string.push(curr_char);
            curr_char = chars.next().unwrap();
        }

        let token = Token::String(string);
        return token;
    }

    pub fn lex(code: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars = code.chars().peekable();

        // if iterator still has content
        while let Some(curr_char) = chars.next() {
            match curr_char {
                '=' => tokens.push(Token::Igual),
                '+' => tokens.push(Token::Suma),
                '-' => tokens.push(Token::Resta),
                '*' => tokens.push(Token::Multiplicacion),
                '/' => tokens.push(Token::Division),

                '<' => {
                    let next_char = chars.peek().unwrap_or(&' ');

                    if next_char == &'-' {
                        tokens.push(Token::Igual);
                        chars.next();
                    } else if next_char == &'=' {
                        chars.next();
                        tokens.push(Token::MenorOIgual)
                    } else {
                        tokens.push(Token::MenorA)
                    }
                }

                '>' => {
                    let next_char = chars.peek().unwrap_or(&' ');

                    if next_char == &'=' {
                        tokens.push(Token::MayorOIgual);
                        chars.next();
                    } else {
                        tokens.push(Token::MayorA)
                    }
                }

                '(' => tokens.push(Token::AbrirParentesis),
                ')' => tokens.push(Token::CerrarParentesis),

                '\"' => {
                    let token = Lexer::parse_string('\"', &mut chars);
                    tokens.push(token);
                }

                '\'' => {
                    let token = Lexer::parse_string('\'', &mut chars);
                    tokens.push(token);
                }

                ch if ch.is_numeric() => {
                    let token = Lexer::parse_numeric(ch, &mut chars);
                    tokens.push(token);
                }

                ch if ch.is_alphanumeric() => {
                    let token = Lexer::parse_alphanumeric(ch, &mut chars);
                    tokens.push(token);
                }

                _ => {
                    // should error out, but we can ignore it for now
                }
            }
        }

        println!("{:?}", tokens);

        tokens
    }
}

pub fn find_algorithm(code: &String) -> (usize, usize) {
    let lines = code.lines().map(|line| line.to_ascii_lowercase());
    let mut algo_start: usize = 0;
    let mut algo_end: usize = 0;

    // TODO: add error handling for files without starting or ending algorithm keywords
    for (idx, line) in lines.enumerate() {
        if line.starts_with("algoritmo") {
            algo_start = idx
        } else if line.starts_with("finalgoritmo") {
            algo_end = idx;
        }
    }

    (algo_start, algo_end)
}
