use std::io::{Read, Write};

pub struct Machine {
    tape: Vec<u8>,
    source: Vec<Instruction>,
    ptr: usize,
    pc: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Inc,
    Dec,
    PtrInc,
    PtrDec,
    LoopBeg,
    LoopEnd,
    Output,
    Input,
}

impl Machine {
    pub fn new(source: Vec<char>) -> Self {
        Machine {
            tape: vec![0u8; 30000],
            source: parse(source),
            ptr: 0,
            pc: 0,
        }
    }

    pub fn run(&mut self) -> std::io::Result<()> {
        while self.pc < self.source.len() {
            match self.source[self.pc] {
                Instruction::Inc => {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(1);
                }
                Instruction::Dec => {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(1);
                }
                Instruction::PtrInc => {
                    self.ptr += 1;
                }
                Instruction::PtrDec => {
                    self.ptr -= 1;
                }
                Instruction::LoopBeg => {
                    if self.tape[self.ptr] == 0 {
                        self.pc =
                            find_closing(self.pc, &self.source).expect("Expected ']' after '['");
                    }
                }
                Instruction::LoopEnd => {
                    if self.tape[self.ptr] != 0 {
                        self.pc = find_opening(self.pc, &self.source)
                            .expect("Expected '[' corresponding to ']'");
                    }
                }
                Instruction::Output => std::io::stdout().write_all(&[self.tape[self.ptr]])?,
                Instruction::Input => {
                    std::io::stdout().flush().unwrap();
                    let mut buf = [0u8; 1];

                    match std::io::stdin().read_exact(&mut buf) {
                        Ok(()) => {
                            self.tape[self.ptr] = buf[0];
                        }
                        Err(_) => {
                            eprintln!("Stdin did not receive one char");
                        }
                    }
                }
            }

            // for now I don't want to implement wrapping for this
            self.pc += 1;
        }
        Ok(())
    }
}

fn parse(source: Vec<char>) -> Vec<Instruction> {
    let mut src: Vec<Instruction> = Vec::new();
    for c in source {
        let instr = match c {
            '>' => Some(Instruction::PtrInc),
            '<' => Some(Instruction::PtrDec),
            '+' => Some(Instruction::Inc),
            '-' => Some(Instruction::Dec),
            '[' => Some(Instruction::LoopBeg),
            ']' => Some(Instruction::LoopEnd),
            '.' => Some(Instruction::Output),
            ',' => Some(Instruction::Input),
            _ => None,
        };
        if let Some(ins) = instr {
            src.push(ins);
        }
    }
    src
}

fn find_opening(start: usize, source: &[Instruction]) -> Option<usize> {
    let mut depth = 1;
    for (index, instruction) in source[0..start].iter().enumerate().rev() {
        match instruction {
            Instruction::LoopEnd => depth += 1,
            Instruction::LoopBeg => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn find_closing(start: usize, source: &[Instruction]) -> Option<usize> {
    let mut depth = 1;

    for (index, instruction) in source.iter().enumerate().skip(start + 1) {
        match instruction {
            Instruction::LoopBeg => depth += 1,
            Instruction::LoopEnd => {
                depth -= 1;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}
