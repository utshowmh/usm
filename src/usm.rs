use std::{
    fs::{read_to_string, write},
    process::exit,
};

use crate::{error::USMError, instruction::InstructionAsByte};

pub struct USM {
    source: String,
    instructions: Vec<(String, Option<String>)>,
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
                let operation = &instruction.0;
                match &instruction.1 {
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
        let instructions = self.source.trim().split("\n");
        for instruction in instructions {
            let instruction: Vec<&str> = instruction.trim().split(" ").collect();
            let instruction_len = instruction.len();
            match instruction_len {
                1 => {
                    let operation = instruction[0].trim();
                    match operation {
                        "pop" => {
                            self.instructions
                                .push((InstructionAsByte::Pop.to_string(), None));
                        }
                        "eql" => {
                            self.instructions
                                .push((InstructionAsByte::Equal.to_string(), None));
                        }
                        "add" => {
                            self.instructions
                                .push((InstructionAsByte::Add.to_string(), None));
                        }
                        "sub" => {
                            self.instructions
                                .push((InstructionAsByte::Subtract.to_string(), None));
                        }
                        "mult" => {
                            self.instructions
                                .push((InstructionAsByte::Multipy.to_string(), None));
                        }
                        "div" => {
                            self.instructions
                                .push((InstructionAsByte::Divide.to_string(), None));
                        }
                        "print" => {
                            self.instructions
                                .push((InstructionAsByte::Print.to_string(), None));
                        }
                        "halt" => {
                            self.instructions
                                .push((InstructionAsByte::Halt.to_string(), None));
                        }
                        _ => {
                            if operation.starts_with("'") {
                                let label_name =
                                    instruction[0].strip_prefix("'").unwrap().trim().to_string();
                                if label_name.ends_with(":") {
                                    let label_name =
                                        label_name.strip_suffix(":").unwrap().trim().to_string();
                                    self.instructions.push((format!("'{}:", label_name), None));
                                } else {
                                    return Some(USMError::IllegalLabel);
                                }
                            } else {
                                return Some(USMError::IllegalToken);
                            }
                        }
                    }
                }
                2 => {
                    let operation = instruction[0].trim();
                    let operand = instruction[1].to_string();
                    match operation {
                        "push" => {
                            let operand: i64 = match instruction[1].trim().parse() {
                                Ok(operand) => operand,
                                Err(err) => {
                                    eprintln!("{:#?}", err);
                                    return Some(USMError::InvalidArgument);
                                }
                            };
                            self.instructions.push((
                                InstructionAsByte::Push.to_string(),
                                Some(operand.to_string()),
                            ));
                        }
                        "dup" => {
                            let operand: i64 = match instruction[1].trim().parse() {
                                Ok(operand) => operand,
                                Err(err) => {
                                    eprintln!("{:#?}", err);
                                    return Some(USMError::InvalidArgument);
                                }
                            };
                            self.instructions.push((
                                InstructionAsByte::Duplicate.to_string(),
                                Some(operand.to_string()),
                            ));
                        }
                        "jmp" => {
                            self.instructions
                                .push((InstructionAsByte::Jump.to_string(), Some(operand)));
                        }
                        "jmpif" => {
                            self.instructions
                                .push((InstructionAsByte::JumpIf.to_string(), Some(operand)));
                        }
                        _ => {
                            return Some(USMError::IllegalToken);
                        }
                    }
                }
                _ => {
                    if instruction_len > 1 && instruction[0].starts_with("#") {
                        continue;
                    } else {
                        println!("here -> {:#?}", instruction);
                        return Some(USMError::InvalidArgument);
                    }
                }
            }
        }
        None
    }
}
