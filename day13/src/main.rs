extern crate intcode;

use intcode::{Computer, Program};
use std::fs;

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day13.txt").unwrap();
    let mut program = Program::from(input);
    let computer = Computer::new(program.clone());

    let blocks = computer.skip(2).step_by(3).filter(|&x| x == 2).count();
    println!("{}", blocks);

    program.set(0, 2);
    let computer = Computer::new(program);
}
