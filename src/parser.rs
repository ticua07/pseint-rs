use std::error::Error;

use crate::utils::handle_functions;

#[derive(Debug)]
pub struct Command {
    pub function: String,
    pub args: Vec<String>,
}

// parser should return a result
// err should be syntax error and Ok() should be Vec<Command>
pub fn parse(input: String) -> Result<Vec<Command>, Box<dyn Error>> {
    let lines: Vec<&str> = input
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect();

    let mut commands: Vec<Command> = Vec::new();

    handle_functions(&lines);

    let mut start_line: usize;
    let mut _end_line: usize;

    for (idx, line) in lines.iter().enumerate() {
        let mut args: Vec<String> = line
            .split_ascii_whitespace()
            .map(|x| x.to_string())
            .collect();
        args.remove(0);

        if line.starts_with("Algoritmo") && args.len() >= 1 {
            start_line = idx;
            commands.push(Command {
                function: "Algoritmo".to_string(),
                args: args.clone(),
            });

            let mut parsed_block = parse_commands(&lines, start_line);

            commands.append(&mut parsed_block);
        }
    }

    Ok(commands)
}

/*
Name may be ambiguous, but this is used to only parse commands
Instead of running the same code to parse functions, algos
just run this function with the children inside the command block

*/
pub fn parse_commands(lines: &Vec<&str>, start_line: usize) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    for algo_lines in start_line..lines.len() {
        if lines[algo_lines].starts_with("FinAlgoritmo") {
            if commands.iter().any(|x| x.function == "FinAlgoritmo") {
                return Err("Multiple algorithms in same program").unwrap();
            }

            commands.push(Command {
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

            commands.push(Command {
                function: "Escribir".to_string(),
                args: args,
            })
        }
    }

    commands
}
