use std::{iter::Peekable, str::Chars};

pub struct Lexer {}

impl Lexer {
    fn is_text(ch: char) -> bool {
        ch.is_alphanumeric()
    }

    fn parse_alphanumeric(initial_char: char, chars: &mut Peekable<Chars>) -> String {
        let mut curr_char = initial_char;
        let mut string = String::new();

        while Lexer::is_text(curr_char) {
            string.push(curr_char);
            curr_char = chars.next().unwrap_or(' ');
        }

        let token = format!("string-{}", string);

        return token;
    }

    pub fn lex(code: String) {
        let mut tokens: Vec<String> = Vec::new();
        let mut chars = code.chars().peekable();

        // if iterator still has content
        while chars.peek().is_some() {
            let curr_char = chars.next().unwrap();

            match curr_char {
                '=' => tokens.push("igual".to_string()),

                ch if Lexer::is_text(ch) => {
                    let token = Lexer::parse_alphanumeric(ch, &mut chars);
                    tokens.push(token);
                }

                _ => {
                    // should error out, but we can ignore it for now
                }
            }
        }

        println!("{:?}", tokens);
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
