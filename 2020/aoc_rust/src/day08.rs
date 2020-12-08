extern crate regex;

use std::vec::Vec;
use regex::Regex;

pub fn run_day08(puzzle_input: &str) {
    let mut program = Program::new(&puzzle_input);
    if let ProgramResult::FoundLoop(pc, acc) = program.compute() {
        println!("Acc before loop: {}, pc={}", acc, pc);
    } else {
        println!("No loop detected.");
    }

    if let Some((_, acc, line)) = program.get_fixed_program() {
        println!("Acc after fixing line {}: {}", line, acc);
    } else {
        println!("Could not fix program :(");
    }
}

#[derive(Clone)]
struct Program {
    instructions: Vec<Instruction>
}

enum ProgramResult {
    FoundLoop(usize, i32), // pc, acc
    Terminated(i32), // acc
    PcOutOfBounds(i32, i32) // pc, acc
}

#[derive(Clone)]
struct Instruction {
    op: Operation,
    call_count: u8
}

#[derive(Clone, Copy)]
enum Operation {
    NOP(i32), // having an arg for NOP is important for code fixing
    ACC(i32),
    JMP(i32)
}

impl Program {
    fn new(assembly_filename: &str) -> Program {
        lazy_static! {
            // (?m) enables "multiline mode":
            static ref INSTRUCTION: Regex = Regex::new(
                r"(?m)^(?P<op>\w+) (?P<arg>[\+\-]?\d+)$"
            ).unwrap();
        }

        let code = std::fs::read_to_string(&assembly_filename)
            .expect("Could not read source assembly code.");
        let mut instructions: Vec<Instruction> = Vec::new();
        for instr_cap in INSTRUCTION.captures_iter(&code) {
            let op: Operation;
            let arg: i32 = instr_cap.name("arg").unwrap().as_str()
                .parse::<i32>().expect("Could not parse arg to i32");
            match instr_cap.name("op").unwrap().as_str() {
                "nop" => { op = Operation::NOP(arg); },
                "acc" => { op = Operation::ACC(arg); },
                "jmp" => { op = Operation::JMP(arg); },
                op => { panic!("Invalid operation \"{}\"", op); }
            }
            instructions.push(Instruction {
                op,
                call_count: 0
            });
        }

        Program {
            instructions
        }
    }

    fn compute(&mut self) -> ProgramResult {
        self.reset();
        let mut acc: i32 = 0; // the accumulator
        let mut pc: i32 = 0; // the program counter

        loop {
            if pc < 0 || pc > self.instructions.len() as i32 {
                return ProgramResult::PcOutOfBounds(pc, acc);
            }
            if pc == self.instructions.len() as i32 {
                return ProgramResult::Terminated(acc);
            }
            let instruction: &mut Instruction = self.instructions
                .get_mut(pc as usize).unwrap();
            if instruction.call_count > 0 {
                return ProgramResult::FoundLoop(pc as usize, acc);
            }
            instruction.call_count += 1;
            match instruction.op {
                Operation::NOP(_) => { pc += 1; },
                Operation::ACC(arg) => { pc += 1; acc += arg; },
                Operation::JMP(arg) => { pc += arg; }
            }
        }
    }

    fn reset(&mut self) {
        for instruction in self.instructions.iter_mut() {
            instruction.call_count = 0;
        }
    }

    /// Returns the first program that terminates successfully
    /// if exactly one NOP/JMP is flipped, plus the final acc value
    /// and the 0-based index of the corrupted instruction.
    fn get_fixed_program(&self) -> Option<(Program, i32, usize)> {
        for i in 0..self.instructions.len() {
            let mut program: Program = self.clone();
            match program.instructions[i].op {
                Operation::NOP(arg) => {
                    program.instructions[i].op = Operation::JMP(arg);
                },
                Operation::JMP(arg) => {
                    program.instructions[i].op = Operation::NOP(arg);
                },
                _ => { continue; }
            }
            if let ProgramResult::Terminated(acc) = program.compute() {
                return Some((program, acc, i));
            }
        }
        None
    }
}
