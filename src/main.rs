use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::num::Wrapping;

type CellData = Wrapping<u8>;

macro_rules! wrap_data {
    ($arg:expr) => {
        Wrapping::<u8>($arg)
    };
}

const VAL_ZERO: CellData = wrap_data!(0);
const VAL_ONE: CellData = wrap_data!(1);
const VAL_MINUS_ONE: CellData = wrap_data!(255);

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

#[derive(Debug)]
struct Jump {
    here: isize, // location/index of instruction
    jump_to: isize, // location/index to conditionally jump to
                 // positive numbers indicate start of loops, negative numbers indicate end of loops
}

#[derive(Debug)]
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
    let input_str = "cool cat :3";
    let program_str = _pg_hello_world;

    let mut program = Vec::<Op>::new();
    let mut jump_index: isize = 1;
    let mut jump_stack = Vec::new();
    // TODO: explicitly check unmatched []
    for c in program_str.chars() {
        match c {
            '+' => program.push(Op::Add(VAL_ONE)),
            '-' => program.push(Op::Sub(VAL_ONE)),
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
                        jump_to: jump_to,
                    }));
                }
                None => {
                    panic!("Unmatched closing bracket");
                }
            },

            _ => (), // all other characters are treated as comments
        }
    }

    let mut jump_map = HashMap::new();
    for (i, op) in program.iter().enumerate() {
        match op {
            Op::JumpIfZero(Jump { here, jump_to: _ })
            | Op::JumpIfNonZero(Jump { here, jump_to: _ }) => {
                jump_map.insert(here, i);
            }
            _ => (),
        }
    }

    debugln!("{:?}", program);
    debugln!("{:?}", jump_map);

    let mut input = VecDeque::<CellData>::new();
    for c in input_str.chars() {
        input.push_back(wrap_data!(c as u8));
    }

    //let mut tape = VecDeque::<u8>::new();
    let mut tape = vec![VAL_ZERO];
    let mut ptr: usize = 0;
    let mut ip: usize = 0;

    let mut op_count = 0;

    while ip < program.len() {
        match program[ip] {
            Op::Add(amt) => tape[ptr] += amt,
            Op::Sub(amt) => tape[ptr] -= amt,
            Op::Move(amt) => {
                // ptr += amt as ussize
                if (amt < 0) & (ptr < -amt as usize) {
                    panic!("instruction pointer underflow at char {}", ip);
                }
                ptr = ptr.wrapping_add(amt as usize);
                while ptr >= tape.len() {
                    tape.push(VAL_ZERO);
                }
            }
            Op::Print => {
                print!("{}", tape[ptr].0 as char);
                io::stdout().flush().unwrap();
            }
            Op::Input => match input.pop_front() {
                Some(v) => tape[ptr] = v,
                None => tape[ptr] = VAL_MINUS_ONE,
            },
            Op::JumpIfZero(Jump { here: _, jump_to }) => {
                if tape[ptr] == VAL_ZERO {
                    ip = jump_map[&jump_to];
                }
            }
            Op::JumpIfNonZero(Jump { here: _, jump_to }) => {
                if tape[ptr] != VAL_ZERO {
                    ip = jump_map[&jump_to];
                }
            }
            //_ => println!("wtf"),
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
