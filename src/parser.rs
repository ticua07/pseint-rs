#[derive(Debug)]
pub struct Function {
    pub function: String,
    pub args: Vec<String>,
}

// parser should return a result
// err should be syntax error and Ok() should be Vec<Function>
pub fn parse(input: String) -> Vec<Function> {
    let lines: Vec<&str> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();

    let mut commands: Vec<Function> = Vec::new();

    let mut _start_line: usize = 0;
    let mut _end_line: usize = 0;
    for (idx, line) in lines.iter().enumerate() {
        let mut args: Vec<&str> = line.split_ascii_whitespace().collect();
        args.remove(0);

        if line.starts_with("Algoritmo") && args.len() >= 1 {
            _start_line = idx;
            for algo_lines in _start_line..lines.len() {
                if line.starts_with("FinAlgoritmo") && args.len() == 0 {
                    _end_line = idx;
                }

                if lines[algo_lines].starts_with("//") {
                    // if comment
                    continue;
                }

                if lines[algo_lines].starts_with("Escribir") {
                    let mut args: Vec<String> = lines[algo_lines]
                        .split_ascii_whitespace()
                        .map(|x| x.to_string())
                        .collect();
                    args.remove(0);

                    commands.push(Function {
                        function: "Escribir".to_string(),
                        args: args,
                    })
                }
            }
        }
    }

    commands
}
