use std::{env::args, process::exit};

use usm::USM;

mod error;
mod instruction;
mod usm;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() == 3 {
        let source_path = &args[1];
        let output_path = &args[2];

        let mut assembler = USM::new();
        assembler.run(source_path, output_path);
    } else {
        eprintln!(
            "
Program: USM

Usage:
    <source_path> <output_path>: executes the (given) file.
        "
        );
        exit(1);
    }
}
