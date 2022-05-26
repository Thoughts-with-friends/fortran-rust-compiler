use fortran_rust_compiler::lexer::parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Incorrect number of arguments.");
    }

    parse(args[1].to_string().as_str())
}
