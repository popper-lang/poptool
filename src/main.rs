extern crate popper;
use popper::{execute_file, get_ast_from_file};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match args[1].as_str() {
            "--help" | "-h" => {
                println!("Usage: popper run [file]");
                println!("If no file is specified, popper will read from stdin");
            }
            "run" => {
                if args.len() >= 3 {
                    match execute_file(&args[2]) {
                        Ok(_) => {}
                        Err(e) => println!("error: {:?}", e),
                    }
                } else {
                    println!("error: no file specified");
                }
            },
            "ast" => {
                if args.len() >= 3 {
                    match get_ast_from_file(&args[2]) {
                        Ok(ast) => println!("{:#?}", ast),
                        Err(e) => println!("error: {:?}", e),
                    }
                } else {
                    println!("error: no file specified");
                }
            },
            _ => {
                println!("error: invalid command");
                println!("Usage: popper run [file]");
                println!("If no file is specified, popper will read from stdin");
            }
        }
    } else {
        println!("Usage: poptool run <file>");
    }
}
