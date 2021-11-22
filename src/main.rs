#![deny(clippy::all, clippy::nursery)]

// TODO:
// - Switch to a recursive Loop op rather than Jumps
// - Actual error handling on interpreter
// - bounded memory
// - IR optimization step
// - Further IR instructions (e.g. set value, multiply, etc.)
// - Special debug print symbols and IR instructions
// - Compilation

use std::collections::HashMap;
use std::io::{self, Write};

type CellData = u8;
const INPUT_END_VAL: CellData = CellData::MAX;

const DEBUG: bool = false;
const MAX_OPS: isize = -1;

macro_rules! debugln {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
            io::stdout().flush().unwrap();
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Jump {
    here: isize, // location/index of instruction
    jump_to: isize, // location/index to conditionally jump to
                 // positive numbers indicate start of loops, negative numbers indicate end of loops
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add(CellData),
    Sub(CellData),
    Move(isize),
    JumpIfZero(Jump),
    JumpIfNonZero(Jump),
    Input,
    Output,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProgramState {
    tape: Vec<CellData>,
    inst_ptr: usize,
    tape_ptr: usize,
    input_ptr: usize,
    pub output: Vec<CellData>,
    halted: bool,
}

impl ProgramState {
    fn interpret(&mut self, program: &[Op], input: &[CellData], max_ops: isize) {
        let jump_map = create_jump_map(&program);
        let mut op_count = 0;

        while self.inst_ptr < program.len() {
            let p = self.tape.get_mut(self.tape_ptr).unwrap();
            match program[self.inst_ptr] {
                Op::Add(amt) => *p = p.wrapping_add(amt),
                Op::Sub(amt) => *p = p.wrapping_sub(amt),
                Op::Move(amt) => {
                    // ptr += amt as ussize
                    if (amt < 0) & (self.tape_ptr < -amt as usize) {
                        panic!("instruction pointer underflow at char {}", self.inst_ptr);
                    }
                    self.tape_ptr = self.tape_ptr.wrapping_add(amt as usize);
                    while self.tape_ptr >= self.tape.len() {
                        self.tape.push(0);
                    }
                }
                Op::Output => {
                    self.output.push(*p);
                }
                Op::Input => {
                    *p = match input.get(self.input_ptr) {
                            Some(v) => {
                                self.input_ptr += 1;
                                *v
                            }
                            None => INPUT_END_VAL,
                        }
                }
                Op::JumpIfZero(Jump { here: _, jump_to }) => {
                    if *p == 0 {
                        self.inst_ptr = jump_map[&jump_to];
                    }
                }
                Op::JumpIfNonZero(Jump { here: _, jump_to }) => {
                    if *p != 0 {
                        self.inst_ptr = jump_map[&jump_to];
                    }
                }
            }
            debugln!("{:?} {:?}", program[self.inst_ptr], self.tape);
            self.inst_ptr += 1;
            op_count += 1;
            if (max_ops >= 0) & (op_count > max_ops) {
                panic!("\nProgram took too long lol");
            }
        }
    }
}

impl std::default::Default for ProgramState {
    fn default() -> Self {
        Self {
            tape: vec![0],
            inst_ptr: 0,
            tape_ptr: 0,
            input_ptr: 0,
            output: Vec::new(),
            halted: false,
        }
    }
}

fn main() {
    //let program_str: &str = "+++[>++<-]++"; [[[]][]][]
    let _pg_hello_world =
        ">>>>>+[-->-[>>+ >-----<<]< --<---]>-.>>>+.>>..+++[.>]<<<<.+++.------.<<-.>>>>+.";
    let _pg_cat = ",+[-.,+]";
    let _pg_overflow = "++++++++[->++++++++<]";
    let input_str = "cool cat :3";
    let program_str = _pg_cat;

    let program = parse(program_str);
    debugln!("{:?}", program);
    let input = Vec::<CellData>::from_iter(input_str.bytes());
    let mut machine = ProgramState::default();
    machine.interpret(&program, &input, MAX_OPS);
    println!("{}", std::str::from_utf8(&machine.output).unwrap());
}

fn parse(program_str: &str) -> Vec<Op> {
    let mut program = Vec::<Op>::new();
    let mut jump_index: isize = 1;
    let mut jump_stack = Vec::new();
    // TODO: explicitly check unmatched []
    for c in program_str.chars() {
        match c {
            '+' => program.push(Op::Add(1)),
            '-' => program.push(Op::Sub(1)),
            '>' => program.push(Op::Move(1)),
            '<' => program.push(Op::Move(-1)),
            '.' => program.push(Op::Output),
            ',' => program.push(Op::Input),
            '[' => {
                program.push(Op::JumpIfZero(Jump {
                    here: jump_index,
                    jump_to: -jump_index,
                }));
                jump_stack.push(jump_index);
                jump_index += 1;
            }
            ']' => match jump_stack.pop() {
                Some(jump_to) => {
                    program.push(Op::JumpIfNonZero(Jump {
                        here: -jump_to,
                        jump_to,
                    }));
                }
                None => {
                    panic!("Unmatched closing bracket");
                }
            },

            _ => (), // all other characters are treated as comments
        }
    }
    program
}


fn create_jump_map(program: &[Op]) -> HashMap<isize, usize> {
    let mut jump_map = HashMap::new();
    for (i, op) in program.iter().enumerate() {
        match op {
            Op::JumpIfZero(Jump { here, jump_to: _ })
            | Op::JumpIfNonZero(Jump { here, jump_to: _ }) => {
                jump_map.insert(*here, i);
            }
            _ => (),
        }
    }
    jump_map
}
