use std::fs;

use run::run;
mod parser;
mod run;
fn main() {
    let input = fs::read_to_string("Algoritmo.psc").unwrap();

    let code = parser::parse(input);

    for output in run(code.unwrap()) {
        print!("{output}");
    }
}
