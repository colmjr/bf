use std::io::{self, Read, Write};

pub struct Machine {
    tape: Vec<u8>,
    source: Vec<Instruction>,
    jump: Vec<Option<usize>>,
    ptr: usize,
    pc: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Inc(usize),
    Dec(usize),
    PtrInc(usize),
    PtrDec(usize),
    LoopBeg,
    LoopEnd,
    Output,
    Input,
}

impl Machine {
    pub fn new(source: Vec<char>) -> Self {
        let source = parse(source);
        let jump: Vec<Option<usize>> = build_jump(&source);
        Machine {
            tape: vec![0u8; 30000],
            source,
            jump,
            ptr: 0,
            pc: 0,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        while self.pc < self.source.len() {
            match self.source[self.pc] {
                Instruction::Inc(n) => {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_add(n as u8);
                }
                Instruction::Dec(n) => {
                    self.tape[self.ptr] = self.tape[self.ptr].wrapping_sub(n as u8);
                }
                Instruction::PtrInc(n) => {
                    self.ptr += n;
                }
                Instruction::PtrDec(n) => {
                    self.ptr -= n;
                }
                Instruction::LoopBeg => {
                    if self.tape[self.ptr] == 0 {
                        self.pc = self.jump[self.pc].unwrap();
                    }
                }
                Instruction::LoopEnd => {
                    if self.tape[self.ptr] != 0 {
                        self.pc = self.jump[self.pc].unwrap();
                    }
                }
                Instruction::Output => io::stdout().write_all(&[self.tape[self.ptr]])?,
                Instruction::Input => {
                    io::stdout().flush().unwrap();
                    let mut buf = [0u8; 1];

                    match io::stdin().read_exact(&mut buf) {
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

fn build_jump(source: &[Instruction]) -> Vec<Option<usize>> {
    let mut jumps = vec![None; source.len()];
    let mut stack: Vec<usize> = Vec::new();

    for (idx, instruction) in source.iter().enumerate() {
        if instruction == &Instruction::LoopBeg {
            stack.push(idx);
        } else if instruction == &Instruction::LoopEnd {
            let beg = stack.pop().expect("Unmatched ']'");
            jumps[beg] = Some(idx);
            jumps[idx] = Some(beg);
        }
    }

    if !stack.is_empty() {
        panic!("Unmatched '['")
    }

    jumps
}

fn parse(source: Vec<char>) -> Vec<Instruction> {
    let mut src: Vec<Instruction> = Vec::new();
    for c in source {
        let instruction = match c {
            '>' => Some(Instruction::PtrInc(1)),
            '<' => Some(Instruction::PtrDec(1)),
            '+' => Some(Instruction::Inc(1)),
            '-' => Some(Instruction::Dec(1)),
            '[' => Some(Instruction::LoopBeg),
            ']' => Some(Instruction::LoopEnd),
            '.' => Some(Instruction::Output),
            ',' => Some(Instruction::Input),
            _ => None,
        };

        if let Some(ins) = instruction {
            match (src.last_mut(), ins) {
                (Some(Instruction::Inc(count)), Instruction::Inc(1)) => {
                    *count += 1;
                }
                (Some(Instruction::Dec(count)), Instruction::Dec(1)) => {
                    *count += 1;
                }
                (Some(Instruction::PtrInc(count)), Instruction::PtrInc(1)) => {
                    *count += 1;
                }
                (Some(Instruction::PtrDec(count)), Instruction::PtrDec(1)) => {
                    *count += 1;
                }
                (_, ins) => {
                    src.push(ins);
                }
            }
        }
    }
    src
}
