extern crate intcode;

use intcode::{Computer, Program};
use std::fs;

enum Beam {
    Outside,
    Inside,
}

struct Bot {
    program: Program,
}

impl Bot {
    fn test(&self, x: usize, y: usize) -> Beam {
        let mut computer = Computer::new(self.program.clone());
        computer.input(x as isize).unwrap();
        computer.input(y as isize).unwrap();
        match computer.output().unwrap() {
            0 => Beam::Outside,
            1 => Beam::Inside,
            _ => panic!("unexpected response"),
        }
    }
}

fn beam_in_square(bot: &Bot, size: usize) -> usize {
    let mut coverage = 0;

    for x in 0..size {
        for y in 0..size {
            coverage += bot.test(x, y) as isize;
        }
    }

    coverage as usize
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day19.txt").unwrap();
    let bot = Bot {
        program: Program::from(input),
    };

    let part1 = beam_in_square(&bot, 50);
    println!("{}", part1);
}
