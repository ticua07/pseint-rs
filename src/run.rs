use crate::parser::Function;

pub fn run(code: Vec<Function>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();

    for line in code {
        if line.function.starts_with("Algoritmo") {
            output.push(format!(
                "**** ejecutando {} ****\n",
                line.args.first().unwrap()
            ));
        }
        if line.function.starts_with("Escribir") {
            output.push(escribir_type(line.args))
        }
        if line.function.starts_with("FinAlgoritmo") {
            output.push("**** ejecucion finalizada ****\n".to_string());

            // finish algo, go to next one
            break;
        }
    }

    output
}

// Handle quotation mark
// handle if output should have newline or not
pub fn escribir_type(input: Vec<String>) -> String {
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
    if parameter.to_lowercase() == "sin saltar;" {
        return string.to_string();
    } else {
        return format!("{string}\n").to_string();
    }
}
