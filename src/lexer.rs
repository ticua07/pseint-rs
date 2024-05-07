enum Keywords {
    Definir,
    Caracter,
}

pub struct Lexer {}

impl Lexer {
    fn is_text(ch: char) -> bool {
        ch.is_alphanumeric()
    }

    fn parse_alphanumeric(code: &String, previous_idx: usize) -> (usize, String) {
        let mut idx = previous_idx;
        let mut string = String::new();
        let mut curr_char = code.chars().nth(idx).expect("a");

        while Lexer::is_text(curr_char) {
            string.push(curr_char);
            idx += 1;
            curr_char = code.chars().nth(idx).unwrap_or(' ');
        }

        let token = format!("string-{}", string);
        return (idx, token);

        // tokens.push(format!("string-{}", string))
    }

    pub fn lex(code: String) {
        // add or substract 1 to remove "Algoritmo" or "FinAlgoritmo"
        let mut tokens: Vec<String> = Vec::new();
        let mut idx: usize = 0;

        while idx < code.len() {
            let curr_char = code.chars().nth(idx).unwrap();
            match curr_char {
                '=' => tokens.push("igual".to_string()),
                ch if Lexer::is_text(ch) => {
                    let (new_idx, token) = Lexer::parse_alphanumeric(&code, idx);
                    idx = new_idx;
                    tokens.push(token);
                }
                _ => {}
            }
            idx += 1;
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
