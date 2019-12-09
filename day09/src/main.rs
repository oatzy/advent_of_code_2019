use std::fs;

extern crate intcode;
use intcode::{Program,Interupt};

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day09.txt").unwrap();
    let mut program = Program::from(input);

    let input = match program.execute() {
        Interupt::Input(input) => input,
        _ => panic!("unexpected interupt")
    };
    program = input.input(1);

    while let Interupt::Output(output) = program.execute() {
        let (prog, value) = output.receive();
        println!("{}", value);
        program = prog;
    }

}
