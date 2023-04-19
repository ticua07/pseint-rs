use crate::utils::{handle_functions, Function};

struct ParseError(String);

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Error: {}", self.0)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum CommandType {
    Algoritmo,
    FinAlgoritmo,
    Escribir,
    RunFunction,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub function: CommandType,
    pub args: Vec<String>,
}

// parser should return a result
// err should be syntax error and Ok() should be Vec<Command>
pub fn parse(input: String) -> Result<(Vec<Command>, Vec<Function>), Box<dyn std::error::Error>> {
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

        if line.starts_with("Algoritmo") && !args.is_empty() {
            start_line = idx;
            commands.push(Command {
                function: CommandType::Algoritmo,
                args: args.clone(),
            });

            let (mut parsed_block, errors) = parse_commands(&lines, start_line);

            if !errors.is_empty() {
                return Err(errors.join("\n").to_string()).unwrap();
            }

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

#[derive(Debug, Clone)]
pub enum VariableType {
    Text,  // String
    Float, // float i32
    Bool,  // bool
    Int,   // i32
}

#[derive(Debug, Clone)]
pub enum VariableContent {
    Text(String),
    Float(f32),
    Bool(bool),
    Int(i32),
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub variable_name: String,
    pub variable_type: VariableType,
    pub content: Option<VariableContent>,
}

pub fn parse_commands(lines: &Vec<String>, start_line: usize) -> (Vec<Command>, Vec<String>) {
    let mut commands: Vec<Command> = Vec::new();
    let mut variables: Vec<Variable> = Vec::new();
    let mut errors: Vec<String> = Vec::new();

    for algo_line in lines.iter().skip(start_line) {
        if algo_line.starts_with("FinAlgoritmo") {
            if commands
                .iter()
                .any(|x| x.function == CommandType::FinAlgoritmo)
            {
                return Err("Multiple algorithms in same program").unwrap();
            }

            commands.push(Command {
                function: CommandType::FinAlgoritmo,
                args: vec![],
            })
        }

        if algo_line.starts_with("//") {
            // if comment
            continue;
        }

        if algo_line.starts_with("Escribir") {
            let mut args: Vec<String> = algo_line
                .split_ascii_whitespace()
                .map(|x| x.to_string())
                .collect();
            args.remove(0);

            commands.push(Command {
                function: CommandType::Escribir,
                args,
            })
        }

        if algo_line.ends_with("();")
            || algo_line.ends_with("()") && !algo_line.starts_with("Funcion")
        {
            let args: Vec<String> = algo_line.split("()").map(|x| x.to_string()).collect();

            commands.push(Command {
                function: CommandType::RunFunction,
                args: vec![args.first().unwrap().to_owned()],
            })
        }

        if algo_line.starts_with("Definir") {
            let mut args: Vec<String> = algo_line
                .split_ascii_whitespace()
                .map(|x| x.to_string())
                .collect();
            args.remove(0);

            let variables_to_define: Vec<String> = args[0].split(",").map(String::from).collect();
            let psc_variable_type = &args[2].trim_end_matches(";").to_string();

            let variable_type = match psc_variable_type.as_ref() {
                "Caracter" => VariableType::Text,
                "Real" => VariableType::Float,
                "Logico" => VariableType::Bool,
                "Entero" => VariableType::Int,
                _ => {
                    errors.push(format!(
                        "Error: Unknown variable type \"{psc_variable_type}\""
                    ));
                    break;
                }
            };

            for var in variables_to_define {
                variables.push(Variable {
                    variable_name: var,
                    variable_type: variable_type.clone(),
                    content: None, // initialize variable empty
                })
            }
            // println!("{:?} are type {variable_type}", variables_to_define);
        }
    }

    // println!("variables: {:#?}", variables);

    (commands, errors)
}
