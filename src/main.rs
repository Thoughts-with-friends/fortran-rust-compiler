// use fortran_rust_compiler::lexer::parse;

use fortran_rust_compiler::lexer::tokenize;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        panic!("Incorrect number of arguments.");
    }

    let text = args[1].to_string();
    let tokenized = tokenize(&text);
    match tokenized {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
        Err(e) => {
            e.chain().for_each(|e| {
                eprintln!("{}", e);
            });
        }
    }
}
