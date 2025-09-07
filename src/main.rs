use std::{fs::File, io::Read};
use clap::{command, Parser as CliParser};
use inkwell::context::Context;
use parser::Parser;
use crate::llvm::LLVM;

mod parser;
mod llvm;

#[derive(CliParser, Debug)]
#[command(name = "hermes")]
#[command(about = "Hermes Compiler", long_about = None)]
struct Cli {
    file: String,
    output: String,
}

fn main() {
    let args = Cli::parse();
    let file_name = args.file;

    if !file_name.ends_with(".hermes") {
        panic!("Invalid file type, must be of file type hs!")
    }

    let mut file = match File::open(&file_name) {
        Ok(file) => file,
        Err(_) => panic!("Unknown file!"),
    };

    let mut source = String::new();

    match file.read_to_string(&mut source) {
        Ok(_) => {
            // Successfully read the file contents
            println!("Successfully read file!");

            let module_name = file_name.split('/')
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string();

            match Parser::init(&source).parse(module_name.clone()) {
                Ok(ast) => {
                    let llvm_ctx = Context::create();

                    let llvm = LLVM::new(&llvm_ctx, &module_name);

                    let ok = llvm.compile(&args.output);
                }
                Err(err) => {
                    panic!("Error during parsing: {:?}", err);
                }
            }
        }
        Err(error) => {
            panic!("Failed to read the file: {}", error);
        }
    }

}
