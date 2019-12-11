use std::collections::HashMap;
use std::fs;

extern crate intcode;
use intcode::{Computer, Program};

type Layout = HashMap<Point, isize>;

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point(isize, isize);

struct Bot {
    position: Point,
    heading: Direction,
}

impl Bot {
    fn new() -> Self {
        Bot {
            position: Point(0, 0),
            heading: Direction::Up,
        }
    }

    fn step(&mut self, turn: isize) -> Point {
        self.heading = match (self.heading, turn) {
            // left
            (Direction::Up, 0) => Direction::Left,
            (Direction::Down, 0) => Direction::Right,
            (Direction::Left, 0) => Direction::Down,
            (Direction::Right, 0) => Direction::Up,
            // right
            (Direction::Up, 1) => Direction::Right,
            (Direction::Down, 1) => Direction::Left,
            (Direction::Left, 1) => Direction::Up,
            (Direction::Right, 1) => Direction::Down,
            (_, t) => panic!("Got unexpected turn {}", t),
        };
        self.position = match self.heading {
            Direction::Up => Point(self.position.0, self.position.1 + 1),
            Direction::Down => Point(self.position.0, self.position.1 - 1),
            Direction::Left => Point(self.position.0 - 1, self.position.1),
            Direction::Right => Point(self.position.0 + 1, self.position.1),
        };
        self.position
    }
}

fn build_layout(mut computer: Computer, input: isize) -> Layout {
    let mut layout = Layout::new();
    let mut bot = Bot::new();
    let mut input = input;

    loop {
        if !computer.input(input).is_ok() {
            // assume halt
            break;
        };
        let colour = computer.output().unwrap();
        let heading = computer.output().unwrap();

        layout.insert(bot.position, colour);

        let position = bot.step(heading);

        input = *layout.get(&position).or(Some(&0)).unwrap();
    }
    layout
}

fn display_layout(layout: &Layout) {
    //find bounds
    let minx = layout.keys().map(|&x| x.0).min().unwrap();
    let miny = layout.keys().map(|&x| x.1).min().unwrap();
    let maxx = layout.keys().map(|&x| x.0).max().unwrap();
    let maxy = layout.keys().map(|&x| x.1).max().unwrap();

    // reverse y because we're printing top-down
    for y in (miny..=maxy).rev() {
        let line: Vec<&str> = (minx..maxx)
            .map(|x| match layout.get(&Point(x, y)).or(Some(&0)).unwrap() {
                0 => "█",
                1 => " ",
                _ => panic!("got unexpected colour"),
            })
            .collect();
        println!("{}", line.join(""));
    }
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day11.txt").unwrap();
    let program = Program::from(input);

    let layout = build_layout(Computer::new(program.clone()), 0);
    println!("{}", layout.len());

    let layout = build_layout(Computer::new(program), 1);
    display_layout(&layout);
}
