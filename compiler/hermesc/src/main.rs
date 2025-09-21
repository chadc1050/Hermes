use std::{fs, fs::File, io::Read};
use std::io::Write;
use std::path::Path;
use std::process::exit;
use clap::{Parser as CliParser};
use inkwell::context::Context;
use hermesc_parser::Parser;
use hermesc_parser::ast::Module;
use hermesc_llvm::LLVM;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const FILE_EXTENSION: &str = ".hs";

#[derive(CliParser, Debug)]
#[command(name = "hermes")]
#[command(about = "Hermes Compiler", long_about = None)]
struct Cli {
    file: String,
    #[arg(short, long, required = false, default_value_t = String::from("./out/"))]
    output: String,
    #[arg(short, long)]
    emit_ast: bool,
}

fn main() {
    let args = Cli::parse();

    println!("Hermes Compiler Version: {}", VERSION);

    let file_name = args.file;

    println!("Using input file: {}", file_name);

    if !file_name.ends_with(FILE_EXTENSION) {
        eprintln!("Invalid file type, must be of file type hs!");
        exit(1);
    }

    let mut file = match File::open(&file_name) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Unknown file!");
            exit(1);
        },
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
                .split(FILE_EXTENSION)
                .collect::<Vec<&str>>()[0];

            match Parser::init(&source) {
                Ok(mut parser) => {
                    match parser.parse(module_name) {
                        Ok(res) => {

                            if !res.errors.is_empty() {
                                eprintln!("Errors occurred while parsing!");
                                exit(1);
                            }

                            println!("Successfully parsed ast!");

                            // Ensure the output directory exists
                            if !Path::new(&args.output).exists() {
                                fs::create_dir_all(args.output.clone()).unwrap();
                            }

                            if args.emit_ast {
                                emit_ast(&res.ast, args.output.clone())
                            }

                            if !compile(module_name, args.output.clone()) {
                                eprintln!("Failed to compile!");
                                exit(1);
                            }
                        }
                        Err(err) => {
                            panic!("Error during parsing: {:?}", err);
                        }
                    }
                }
                Err(e) => panic!("{:?}", e),
            }
        }
        Err(error) => {
            panic!("Failed to read the file: {}", error);
        }
    }

    fn compile(module_name: &str, output: String) -> bool {
        let llvm_ctx = Context::create();
        let llvm = LLVM::new(&llvm_ctx, module_name);
        let binary_file_name = output + "/" + module_name;
        llvm.compile(&binary_file_name)
    }

    fn emit_ast(ast: &Module, output: String) {
        println!("Emitting AST!");
        let serialized = serde_json::to_string_pretty(&ast).unwrap();
        let ast_file_name = output + "/ast.json";
        println!("Writing ast {:?}", ast_file_name);

        let mut file = File::create(ast_file_name).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }
}