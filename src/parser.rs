use std::error::Error;

use crate::utils::{handle_functions, Function};

#[derive(Debug, Clone)]
pub struct Command {
    pub function: String,
    pub args: Vec<String>,
}

// parser should return a result
// err should be syntax error and Ok() should be Vec<Command>
pub fn parse(input: String) -> Result<(Vec<Command>, Vec<Function>), Box<dyn Error>> {
    let lines: Vec<String> = input
        .lines()
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();

    let mut commands: Vec<Command> = Vec::new();

    let functions = handle_functions(&lines);

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

    Ok((commands, functions))
}

/*
Name may be ambiguous, but this is used to only parse commands
Instead of running the same code to parse functions, algos
just run this function with the children inside the command block

*/
pub fn parse_commands(lines: &Vec<String>, start_line: usize) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    // !!!: REWRITE TO USE ENUMS NOT STRINGS!
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

        if lines[algo_lines].ends_with("();")
            || lines[algo_lines].ends_with("()") && !lines[algo_lines].starts_with("Funcion")
        {
            let args: Vec<String> = lines[algo_lines]
                .split("()")
                .map(|x| x.to_string())
                .collect();

            commands.push(Command {
                function: "PRIV_RUN_FUNCTION".to_string(),
                args: vec![args.first().unwrap().to_owned()],
            })
        }
    }

    // println!("commands: {:#?}", commands);

    commands
}
