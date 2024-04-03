use std::{fs::File, io::Read};

use clap::{command, Parser as CliParser};

use parser::Parser;

mod parser;

#[derive(CliParser, Debug)]
#[command(name = "hermes")]
#[command(about = "Hermes compiler", long_about = None)]
struct Cli {
    file: String,
}

fn main() {
    let args = Cli::parse();
    let file = args.file;

    if !file.ends_with(".hs") {
        panic!("Invalid file type, must be of file type hs!")
    }

    let mut file = match File::open(file) {
        Ok(file) => file,
        Err(_) => panic!("Unknown file!"),
    };

    let mut source = String::new();
    match file.read_to_string(&mut source) {
        Ok(_) => {
            // Successfully read the file contents
            println!("Successfully read file!");
        }
        Err(error) => {
            panic!("Failed to read the file: {}", error);
        }
    }

    let parser = Parser::init(&source);
    parser.parse()
}
