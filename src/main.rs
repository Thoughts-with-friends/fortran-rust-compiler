use std::fs::read_to_string;

use fortran_rust_compiler::parser::parse;

fn main() {
    let unparsed_text = read_to_string("examples/main.f90").unwrap();

    let _parsed_text = parse(&unparsed_text);
}
