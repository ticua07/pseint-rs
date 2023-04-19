use crate::{
    parser::{parse_commands, Command},
    utils::Function,
};

pub fn run(code: Vec<Command>, functions: Vec<Function>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for line in code {
        if line.function.eq(&crate::parser::CommandType::Algoritmo) {
            output.push(format!(
                "**** ejecutando {} ****\n",
                line.args.first().unwrap()
            ));
        }
        if line.function.eq(&crate::parser::CommandType::Escribir) {
            output.push(escribir_type(&line.args)); // Corrected borrow error here
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
                    let func_code = parse_commands(&func.body, 0);

                    let mut func_output = run(func_code, functions.clone());

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
pub fn escribir_type(input: &[String]) -> String {
    let args: String = input.join(" ");
    // Find the index of the first double quote
    let start_quote_index = args.find('"').unwrap_or(0);

    // Find the index of the second double quote
    let end_quote_index = args[start_quote_index + 1..]
        .find('"')
        .map(|index| start_quote_index + 1 + index)
        .unwrap_or(0);

    // Extract the string between the double quotes
    let string = &args[start_quote_index + 1..end_quote_index];

    // Find the index of the first space after the second double quote
    let space_index = args[end_quote_index + 1..]
        .find(' ')
        .map(|index| end_quote_index + 1 + index)
        .unwrap_or(args.len());

    // Extract the parameter after the string
    let parameter = &args.get(space_index + 1..args.len()).unwrap_or("");

    if parameter.to_lowercase().ends_with("sin saltar")
        || parameter.to_lowercase().ends_with("sin saltar;")
    {
        string.to_string()
    } else {
        format!("{string}\n")
    }
}
