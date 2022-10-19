use std::{env::args, process::exit};

use usm::USM;

mod error;
mod instruction;
mod usm;

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        3 => {
            let source_path = &args[1];
            let output_path = &args[2];

            let mut assembler = USM::new();
            assembler.run(source_path, output_path);
        }
        2 => {
            let mut source_path: &str = &args[1];
            if source_path.starts_with("./") {
                source_path = source_path.strip_prefix("./").unwrap();
            } else if source_path.starts_with(".\\") {
                source_path = source_path.strip_prefix(".\\").unwrap();
            }
            let output_path: Vec<&str> = source_path.split(".").collect();
            let mut output_path: String = output_path[0].try_into().unwrap();
            output_path.push_str(".uvm");

            let mut assembler = USM::new();
            assembler.run(source_path, &output_path);
        }
        _ => {
            eprintln!(
                "
Program: USM
    
Usage:
    <source_path>.
    <source_path> <output_path>.
            "
            );
            exit(1);
        }
    }
}
