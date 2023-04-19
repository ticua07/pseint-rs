use std::{fs};

use run::run;

mod parser;
mod run;
mod utils;
fn main() {
    let input = fs::read_to_string("Algoritmo.psc").unwrap();

    let code = parser::parse(input).unwrap();

    for output in run(code.0, code.1) {
        print!("{output}");
    }
}

// let tc: Vec<String> = vec![
//     "Funcion hola2 ()",
//     "Escribir \"Hola mundo\";",
//     "Escribir \"hola world\";",
//     "FinFuncion",
// ]
// .iter()
// .map(|x| x.to_string())
// .collect();
// let funcs = vec![
//     Function {
//         name: "hola".to_string(),
//         parameters: vec!["var1", "var2"]
//             .iter()
//             .map(|x| x.to_string())
//             .collect::<Vec<String>>(),
//         body: vec![
//             "Funcion hola ( var1, var2 )",
//             "Escribir \"Hola mundo123\";",
//             "FinFuncion",
//         ]
//         .iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>(),
//         start: 0,
//         end: 2,
//     },
//     Function {
//         name: "hola2".to_string(),
//         parameters: vec![],
//         body: vec![
//             "Funcion hola2 ()",
//             "Escribir \"Hola mundo\";",
//             "Escribir \"hola world\";",
//             "FinFuncion",
//         ]
//         .iter()
//         .map(|x| x.to_string())
//         .collect::<Vec<String>>(),
//         start: 8,
//         end: 11,
//     },
// ];
// let test = parse_commands(&tc, 100);
// println!("test: {:#?}", test);
// let test_output = run(test, funcs);
// println!("test_o {:#?}", test_output);
