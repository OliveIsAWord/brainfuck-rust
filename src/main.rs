use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, Write};

type CellData = u8;
const INPUT_END_VAL: CellData = CellData::MAX;

const DEBUG: bool = false;
const MAX_OPS: i64 = -1;

macro_rules! debugln {
    ($($arg:tt)*) => {
        if DEBUG {
            println!($($arg)*);
            io::stdout().flush().unwrap();
        }
    }
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }

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
    Print,
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
    let jump_map = create_jump_map(&program);

    debugln!("{:?}", program);
    debugln!("{:?}", jump_map);

    let mut input = VecDeque::<CellData>::new();
    for c in input_str.bytes() {
        input.push_back(c);
    }

    //let mut tape = VecDeque::<u8>::new();
    let mut tape: Vec<CellData> = vec![0];
    let mut ptr: usize = 0;
    let mut ip: usize = 0;

    let mut op_count = 0;

    while ip < program.len() {
        match program[ip] {
            Op::Add(amt) => tape[ptr] = tape[ptr].wrapping_add(amt),
            Op::Sub(amt) => tape[ptr] = tape[ptr].wrapping_sub(amt),
            Op::Move(amt) => {
                // ptr += amt as ussize
                if (amt < 0) & (ptr < -amt as usize) {
                    panic!("instruction pointer underflow at char {}", ip);
                }
                ptr = ptr.wrapping_add(amt as usize);
                while ptr >= tape.len() {
                    tape.push(0);
                }
            }
            Op::Print => {
                print!("{}", tape[ptr] as char);
                io::stdout().flush().unwrap();
            }
            Op::Input => match input.pop_front() {
                Some(v) => tape[ptr] = v,
                None => tape[ptr] = INPUT_END_VAL,
            },
            Op::JumpIfZero(Jump { here: _, jump_to }) => {
                if tape[ptr] == 0 {
                    ip = jump_map[&jump_to];
                }
            }
            Op::JumpIfNonZero(Jump { here: _, jump_to }) => {
                if tape[ptr] != 0 {
                    ip = jump_map[&jump_to];
                }
            }
        }
        debugln!("{:?} {:?}", program[ip], tape);
        ip += 1;
        op_count += 1;
        if (MAX_OPS > 0) & (op_count > MAX_OPS) {
            println!("\nProgram took too long lol");
            break;
        }
    }
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
            '.' => program.push(Op::Print),
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
