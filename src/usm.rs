use std::{
    fs::{read_to_string, write},
    process::exit,
};

use crate::{error::USMError, instruction::InstructionAsByte};

pub struct USM {
    source: String,
    instructions: Vec<(u8, Option<i64>)>,
}

impl USM {
    pub fn new() -> Self {
        Self {
            source: String::new(),
            instructions: Vec::new(),
        }
    }

    pub fn run(&mut self, source_path: &str, output_path: &str) {
        self.source = read_to_string(source_path)
            .unwrap_or_else(|err| {
                eprintln!("USMError: {:#?}", err);
                exit(1);
            })
            .trim()
            .to_string();
        if let Some(err) = self.scan() {
            eprintln!("USMError: {:#?}", err);
            exit(1);
        } else {
            let mut contents = String::new();
            for instruction in &self.instructions {
                let operation = instruction.0;
                match instruction.1 {
                    Some(operand) => contents.push_str(&format!("{} {}\n", operation, operand)),
                    None => contents.push_str(&format!("{}\n", operation)),
                };
            }
            write(output_path, contents).unwrap_or_else(|err| {
                eprintln!("USMError: {:#?}", err);
                exit(1);
            });
        }
    }

    fn scan(&mut self) -> Option<USMError> {
        let instructions = self.source.split("\n");
        for instruction in instructions {
            let instruction: Vec<&str> = instruction.split(" ").collect();
            match instruction.len() {
                1 => {
                    let operation = instruction[0].trim();
                    match operation {
                        "pop" => {
                            self.instructions.push((InstructionAsByte::Pop, None));
                        }
                        "eql" => {
                            self.instructions.push((InstructionAsByte::Equal, None));
                        }
                        "add" => {
                            self.instructions.push((InstructionAsByte::Add, None));
                        }
                        "sub" => {
                            self.instructions.push((InstructionAsByte::Subtract, None));
                        }
                        "mult" => {
                            self.instructions.push((InstructionAsByte::Multipy, None));
                        }
                        "div" => {
                            self.instructions.push((InstructionAsByte::Divide, None));
                        }
                        "dump" => {
                            self.instructions.push((InstructionAsByte::Dump, None));
                        }
                        "hult" => {
                            self.instructions.push((InstructionAsByte::Hult, None));
                        }
                        _ => {
                            return Some(USMError::IllegalToken);
                        }
                    }
                }
                2 => {
                    let operation = instruction[0].trim();
                    let operand: i64 = match instruction[1].trim().parse() {
                        Ok(operand) => operand,
                        Err(err) => {
                            eprintln!("{:#?}", err);
                            return Some(USMError::InvalidArgument);
                        }
                    };
                    match operation {
                        "push" => {
                            self.instructions
                                .push((InstructionAsByte::Push, Some(operand)));
                        }
                        "dup" => {
                            self.instructions
                                .push((InstructionAsByte::Duplicate, Some(operand)));
                        }
                        "jmp" => {
                            self.instructions
                                .push((InstructionAsByte::Jump, Some(operand)));
                        }
                        "jmpif" => {
                            self.instructions
                                .push((InstructionAsByte::JumpIf, Some(operand)));
                        }
                        _ => {
                            return Some(USMError::IllegalToken);
                        }
                    }
                }
                _ => {
                    return Some(USMError::InvalidArgument);
                }
            }
        }
        None
    }
}
