use std::fs;

extern crate intcode;
use intcode::{Interupt, Program};

fn run_program(mut program: Program, mode: isize) -> isize {
    let mut value = 0;

    let input = match program.execute() {
        Interupt::Input(input) => input,
        _ => panic!("unexpected interupt"),
    };
    program = input.input(mode);

    while let Interupt::Output(output) = program.execute() {
        let (prog, val) = output.receive();
        // println!("{}", val);
        program = prog;
        value = val
    }
    value
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day09.txt").unwrap();
    let program = Program::from(input);

    let part1 = run_program(program.clone(), 1);
    println!("{}", part1);

    let part2 = run_program(program.clone(), 2);
    println!("{}", part2);
}
