use std::error::Error;

#[derive(Debug)]
pub struct Function {
    pub function: String,
    pub args: Vec<String>,
}

// parser should return a result
// err should be syntax error and Ok() should be Vec<Function>
pub fn parse(input: String) -> Result<Vec<Function>, Box<dyn Error>> {
    let lines: Vec<&str> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();

    let mut commands: Vec<Function> = Vec::new();

    let mut _start_line: usize = 0;
    let mut _end_line: usize = 0;
    for (idx, line) in lines.iter().enumerate() {
        let mut args: Vec<String> = line
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();
        args.remove(0);

        if line.starts_with("Algoritmo") && args.len() >= 1 {
            _start_line = idx;
            commands.push(Function {
                function: "Algoritmo".to_string(),
                args: args.clone(),
            });
            for algo_lines in _start_line..lines.len() {
                if lines[algo_lines].starts_with("FinAlgoritmo") {
                    if commands.iter().any(|x| x.function == "FinAlgoritmo") {
                        return Err("Multiple algorithms in same program").unwrap();
                    }

                    _end_line = idx;

                    commands.push(Function {
                        function: "FinAlgoritmo".to_string(),
                        args: vec![],
                    })
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

    Ok(commands)
}
