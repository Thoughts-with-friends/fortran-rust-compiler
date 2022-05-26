use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;

// enum TokenKind {
//     Tk_Reserved,
//     Tk_Num,
//     Tk_EOF,
// }

// struct Token {
//     kind: TokenKind,
//     next: String,
//     val: i32,
//     str: String,
// }

pub fn compile_if() {
    let path: &str = "./examples/if.f90";
    let unparsed_file: String = std::fs::read_to_string(path).expect("Fail to read file.");
    // println!("{:?}", unparsed_file);


    // Tokenize
    unparsed_file.split("\n");
    let tokens: Vec<&str> = unparsed_file.split(" ").collect();
    // tokens.push(unparsed_file[0]);
    println!("{:?}", tokens);

    // Create file
    let file_path: String = format!("./examples/if.rs");
    let file = File::create(file_path).unwrap();
    let mut w = BufWriter::new(file);

    // Compile
    for i in 0..tokens.len() {
        if tokens[i].to_string() == "program" {
            write!(w, "fn main() {{\n",).unwrap();
            write!(w, "    ").unwrap();
        } else if tokens[i].to_string() == "if" {
            write!(w, "if {{\n").unwrap();
        } else if tokens[i].to_string() == "endif"
            || tokens[i].to_string() == "end" && &tokens[i + 2].to_string() == "if"
        {
            write!(w, "    ").unwrap();
            write!(w, "}}\n").unwrap();
        }
        // except for "implicit none" and " "
        else if tokens[i].to_string() == "implicit" && &tokens[i + 2].to_string() == "none"
            || tokens[i].to_string() == " "
            || tokens[i].to_string() == "stop"
        {
            continue;
        } else if tokens[i].to_string() == "end" && tokens[i + 2].clone().to_string() == "program\n" {
            write!(w, "}}").unwrap();
        }
        // println!("{:?}", i);
    }
}
