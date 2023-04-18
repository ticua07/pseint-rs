#[derive(Debug)]
pub struct Function {
    name: String,
    parameters: Vec<String>,
    body: String,
}

pub fn handle_functions(lines: &Vec<&str>) {
    let mut functions: Vec<Function> = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        if line.starts_with("Funcion") {
            // let mut args: Vec<String> = line
            //     .split_ascii_whitespace()
            //     .map(|x| x.to_string())
            //     .collect();
            // args.remove(0);

            if let Some((function_name, function_params)) = line.split_once('(') {
                let function_params: Vec<String> = function_params
                    .trim_end_matches(')')
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                functions.push(Function {
                    name: function_name
                        .split_ascii_whitespace()
                        .last()
                        .unwrap() // !: HANDLE LATER
                        .trim()
                        .to_string(),
                    parameters: function_params,
                    body: String::new(),
                });
            }
        }
    }
    println!("found func: {:#?}", functions);
}
