extern crate intcode;

use intcode::{Computer, Program};
use std::fs;

#[derive(Clone, Copy)]
struct P(usize, usize);

#[derive(Eq, PartialEq)]
enum Beam {
    Outside,
    Inside,
}

struct Square {
    top_left: P,
    size: usize,
}

impl Square {
    fn new(size: usize) -> Self {
        Square {
            top_left: P(0, 0),
            size: size,
        }
    }

    fn top_right(&self) -> P {
        P(self.top_left.0 + self.size - 1, self.top_left.1)
    }

    fn bottom_left(&self) -> P {
        P(self.top_left.0, self.top_left.1 + self.size - 1)
    }

    fn right(&mut self) {
        self.top_left = P(self.top_left.0 + 1, self.top_left.1)
    }

    fn down(&mut self) {
        self.top_left = P(self.top_left.0, self.top_left.1 + 1)
    }
}

struct Bot {
    program: Program,
}

impl Bot {
    fn test(&self, position: P) -> Beam {
        let mut computer = Computer::new(self.program.clone());
        computer.input(position.0 as isize).unwrap();
        computer.input(position.1 as isize).unwrap();
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
            coverage += bot.test(P(x, y)) as isize;
        }
    }

    coverage as usize
}

fn find_fit(bot: &Bot, square: &mut Square) -> P {
    // the trick here is, for the square to be fully inside the beam
    // it's sufficient to check only the top-right and bottom-left corners
    // so we track down and right until we reach a point
    // those are both inside, then return the location of the top-left
    loop {
        while bot.test(square.top_right()) != Beam::Inside {
            square.down();
        }

        while bot.test(square.bottom_left()) != Beam::Inside {
            square.right();
        }

        if bot.test(square.top_right()) == Beam::Inside {
            break;
        }
    }
    square.top_left
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day19.txt").unwrap();
    let bot = Bot {
        program: Program::from(input),
    };

    // let part1 = beam_in_square(&bot, 50);
    // println!("{}", part1);

    let mut square = Square::new(100);
    let part2 = find_fit(&bot, &mut square);
    println!("{}", part2.0 * 10_000 + part2.1);
}
