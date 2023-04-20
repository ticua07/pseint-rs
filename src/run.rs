use crate::{
    parser::{parse_commands, Command, Variable, VariableContent},
    utils::Function,
};

pub fn run(code: Vec<Command>, functions: Vec<Function>, variables: Vec<Variable>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for line in code {
        if line.function.eq(&crate::parser::CommandType::Algoritmo) {
            output.push(format!(
                "**** ejecutando {} ****\n",
                line.args.first().unwrap()
            ));
        }
        if line.function.eq(&crate::parser::CommandType::Escribir) {
            output.push(escribir_type(&line.args, variables.clone())); // Corrected borrow error here
        }

        if line.function.eq(&crate::parser::CommandType::FinAlgoritmo) {
            output.push("**** ejecucion finalizada ****\n".to_string());

            // finish algo, go to next one
            break;
        }

        if line.function.eq(&crate::parser::CommandType::RunFunction) {
            for func in &functions {
                if func.name == *line.args.first().unwrap() {
                    // Why 0 you may ask?
                    // Well basically only the function code is sent
                    // so function starts at 0
                    // took me longer than I want to admit to figure it out
                    let (func_code, errors, func_vars) = parse_commands(&func.body, 0);

                    if !errors.is_empty() {
                        return errors;
                    }

                    let mut func_output = run(func_code, functions.clone(), func_vars);

                    output.append(&mut func_output);

                    // output.push(func_output.join("\n").to_string());
                }
            }
        }
    }

    output
}

// Handle quotation mark
// handle if output should have newline or not
pub fn escribir_type(input: &Vec<String>, variables: Vec<Variable>) -> String {
    // ! More info on: parser.rs:129
    let mut args = input.first().unwrap().trim_end_matches(";").to_string();
    if args.to_lowercase().ends_with("sin saltar") {
        args.truncate(args.len() - 11);
        let mut output = replace_variables(args, variables.clone());
        output
    } else {
        let mut output = replace_variables(args.clone(), variables.clone());
        output.push_str("\n");
        output
    }
}

pub fn replace_variables(input: String, variables: Vec<Variable>) -> String {
    let mut args: Vec<String> = input.split(",").map(String::from).collect();

    args = args
        .iter()
        .map(|x| {
            x.trim_start_matches("\"")
                .trim_end_matches("\"")
                .to_string()
        })
        .collect();

    let mut output = args.clone();

    for (idx, arg) in args.clone().iter().enumerate() {
        for var in &variables {
            if arg.trim() == var.variable_name {
                output[idx] = match var.content.to_owned().unwrap() {
                    VariableContent::Text(text) => {
                        // Handle Text variant
                        // You can access the inner String value with `text` variable
                        // and perform the necessary operations
                        text
                    }
                    VariableContent::Int(int) => {
                        // Handle Int variant
                        // You can access the inner i32 value with `int` variable
                        // and perform the necessary operations
                        int.to_string()
                    }
                    VariableContent::Float(float) => {
                        // Handle Float variant
                        // You can access the inner f32 value with `float` variable
                        // and perform the necessary operations
                        float.to_string()
                    }
                    VariableContent::Bool(bool) => {
                        // Handle Bool variant
                        // You can access the inner bool value with `bool` variable
                        // and perform the necessary operations
                        bool.to_string()
                    }
                };
            }
        }
    }

    //     if !arg.trim().starts_with("\"")
    //     && !arg.trim().ends_with("\"")
    //     && var.variable_name == arg
    // {
    //     println!("Found a match: {}", var.variable_name)
    // }

    output.join("")
}
