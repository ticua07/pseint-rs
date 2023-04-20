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
pub fn parse(
    input: String,
) -> Result<(Vec<Command>, Vec<Function>, Vec<Variable>), Box<dyn std::error::Error>> {
    let lines: Vec<String> = input
        .lines()
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();

    let mut commands: Vec<Command> = Vec::new();
    let mut variables: Vec<Variable> = Vec::new();

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

            let (mut parsed_block, errors, local_variables) = parse_commands(&lines, start_line);

            for var in local_variables {
                if !variables.contains(&var) {
                    variables.push(var)
                }
            }

            if !errors.is_empty() {
                return Err(errors.join("\n").to_string()).unwrap();
            }

            commands.append(&mut parsed_block);
        }
    }

    Ok((commands, functions, variables))
}

/*
Name may be ambiguous, but this is used to only parse commands
Instead of running the same code to parse functions, algos
just run this function with the children inside the command block
*/

#[derive(Debug, Clone, PartialEq)]
pub enum VariableType {
    Text,  // String
    Float, // float i32
    Bool,  // bool
    Int,   // i32
}

#[derive(Debug, Clone, PartialEq)]
pub enum VariableContent {
    Text(String),
    Float(f32),
    Bool(bool),
    Int(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub variable_name: String,
    pub variable_type: VariableType,
    pub content: Option<VariableContent>,
}

pub fn parse_commands(
    lines: &Vec<String>,
    start_line: usize,
) -> (Vec<Command>, Vec<String>, Vec<Variable>) {
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

            // Passing the arg as a string, so it's easier to manipulate later
            let string_args: Vec<String> = vec![args.join(" ")];

            commands.push(Command {
                function: CommandType::Escribir,
                args: string_args,
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
                "Entero" => VariableType::Int,
                "Real" => VariableType::Float,
                "Logico" => VariableType::Bool,
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
                });
            }
            // println!("{:?} are type {variable_type}", variables_to_define);
        }

        // check if assignment is happening {var} <- content
        let variable_assignation = &algo_line.splitn(2, "<-").collect::<Vec<&str>>();
        if variable_assignation.len() == 2 {
            let variable_name = variable_assignation[0].trim();
            let value = variable_assignation[1].trim().trim_end_matches(";");

            let variable_type = parse_variable_value(value).unwrap();

            for (idx, var) in &mut variables.to_owned().iter().enumerate() {
                if var.variable_name == variable_name {
                    if var.variable_type
                        == convert_variable_content_to_variable_type(&variable_type)
                    {
                        variables[idx].content = Some(variable_type.clone());
                    }
                }
            }
        }
    }

    (commands, errors, variables)
}
fn convert_variable_content_to_variable_type(content: &VariableContent) -> VariableType {
    match content {
        VariableContent::Text(_) => VariableType::Text,
        // remember always to check for int first, otherwise it will always return float
        VariableContent::Int(_) => VariableType::Int,
        VariableContent::Float(_) => VariableType::Float,
        VariableContent::Bool(_) => VariableType::Bool,
    }
}

fn parse_variable_value(value: &str) -> Result<VariableContent, Box<dyn std::error::Error>> {
    let variable_content = if value.starts_with('\"') && value.ends_with('\"') {
        // If yes, create Text variant without the quotes
        Ok(VariableContent::Text(value[1..value.len() - 1].to_string()))
    } else {
        // If no, try to parse the value into different data types

        value
            .parse::<i32>()
            .map(VariableContent::Int)
            .or_else(|_| value.parse::<bool>().map(VariableContent::Bool))
            .or_else(|_| value.parse::<f32>().map(VariableContent::Float))
            .or_else(|_| Err(format!("Variable type doesn't match {}'s type", value)).unwrap())
        // change unwrap_or_else to a function
    };

    variable_content
}
