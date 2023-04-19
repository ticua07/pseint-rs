#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<String>,
    pub start: usize,
    pub end: usize,
}

pub fn handle_functions(lines: &[String]) -> Vec<Function> {
    let mut functions: Vec<Function> = Vec::new();

    let mut start_line: usize;

    for (idx, line) in lines.iter().enumerate() {
        if line.starts_with("Funcion") {
            start_line = idx;

            if let Some((function_name, function_params)) = line.split_once('(') {
                let args: Vec<String> = function_params
                    .trim_end_matches(')')
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();

                let mut body: Vec<String> = Vec::new();

                for (func_lines, value) in lines.iter().enumerate().skip(start_line) {
                    if lines[func_lines].starts_with("FinFuncion") {
                        // don't forget to set the FinFuncion actual command
                        body.push(value.to_string());
                        functions.push(Function {
                            name: function_name
                                .split_ascii_whitespace()
                                .last()
                                .unwrap() // !: HANDLE LATER
                                .trim()
                                .to_string(),
                            parameters: args.clone(),
                            body: body.clone(),
                            start: start_line,
                            end: func_lines,
                        });
                        // Break here, so that it doesn't
                        // catch other FinFuncion

                        body.clear();

                        break;
                    } else {
                        body.push(value.to_string())
                    }
                }
            }
        }
    }

    functions
}

// let test = parser::parse_commands(&lines, start_line);
// println!("{:#?}", test);
