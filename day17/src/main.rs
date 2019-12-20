extern crate intcode;

use intcode::{Computer, Program};
use std::collections::HashSet;
use std::fmt;
use std::fs;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct P(isize, isize);

impl P {
    fn adjacent(&self) -> Vec<P> {
        vec![
            P(self.0, self.1 + 1),
            P(self.0, self.1 - 1),
            P(self.0 + 1, self.1),
            P(self.0 - 1, self.1),
        ]
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Tile {
    Scaffold = 35,
    Empty = 46,
}

#[derive(Debug)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Forward(isize),
}

#[derive(Clone, Copy)]
enum Facing {
    Up = 94,
    Down = 118,
    Left = 60,
    Right = 62,
}

struct Bot {
    position: P,
    facing: Facing,
}

impl Bot {
    fn forward(&mut self, steps: isize) {
        self.position = match self.facing {
            Facing::Up => P(self.position.0, self.position.1 + steps),
            Facing::Down => P(self.position.0, self.position.1 - steps),
            Facing::Left => P(self.position.0 - steps, self.position.1),
            Facing::Right => P(self.position.0 + steps, self.position.1),
        }
    }

    fn lookahead(&self) -> P {
        match self.facing {
            Facing::Up => P(self.position.0, self.position.1 + 1),
            Facing::Down => P(self.position.0, self.position.1 - 1),
            Facing::Left => P(self.position.0 - 1, self.position.1),
            Facing::Right => P(self.position.0 + 1, self.position.1),
        }
    }

    fn look_left(&self) -> P {
        match self.facing {
            Facing::Up => P(self.position.0 - 1, self.position.1),
            Facing::Down => P(self.position.0 + 1, self.position.1),
            Facing::Left => P(self.position.0, self.position.1 - 1),
            Facing::Right => P(self.position.0, self.position.1 + 1),
        }
    }

    fn look_right(&self) -> P {
        match self.facing {
            Facing::Up => P(self.position.0 + 1, self.position.1),
            Facing::Down => P(self.position.0 - 1, self.position.1),
            Facing::Left => P(self.position.0, self.position.1 + 1),
            Facing::Right => P(self.position.0, self.position.1 - 1),
        }
    }

    fn go(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::TurnLeft => {
                self.facing = match self.facing {
                    Facing::Up => Facing::Left,
                    Facing::Down => Facing::Right,
                    Facing::Left => Facing::Down,
                    Facing::Right => Facing::Up,
                }
            }
            Instruction::TurnRight => {
                self.facing = match self.facing {
                    Facing::Up => Facing::Right,
                    Facing::Down => Facing::Left,
                    Facing::Left => Facing::Up,
                    Facing::Right => Facing::Down,
                }
            }
            Instruction::Forward(steps) => self.forward(steps),
        }
    }
}

struct Image {
    layout: HashSet<P>,
    bot: Bot,
}

impl Image {
    fn read(computer: Computer) -> Image {
        let mut layout = HashSet::new();
        let mut bot = None;

        let mut x = 0;
        let mut y = 0;

        for out in computer {
            match out as u8 as char {
                '\n' => {
                    y += 1;
                    x = 0;
                    continue;
                }
                '#' => {
                    layout.insert(P(x, y));
                }
                '.' => {}
                '^' | 'v' | '<' | '>' => {
                    bot = Some(Bot {
                        position: P(x, y),
                        facing: match out as u8 as char {
                            '^' => Facing::Up,
                            'v' => Facing::Down,
                            '<' => Facing::Left,
                            '>' => Facing::Right,
                            _ => panic!("unexpected"),
                        },
                    });
                    layout.insert(P(x, y));
                }
                c => panic!("unknown character {}", c),
            }
            x += 1;
        }

        Image {
            layout: layout,
            bot: bot.unwrap(),
        }
    }

    fn is_intersection(&self, pos: &P) -> bool {
        pos.adjacent()
            .iter()
            .chain(&[*pos])
            .all(|x| self.layout.contains(x))
    }

    fn calibration(&self) -> isize {
        self.layout
            .iter()
            .filter(|&x| self.is_intersection(x))
            .map(|x| x.0 * x.1)
            .sum()
    }

    fn trace_path(&mut self) -> Vec<Instruction> {
        let mut path = Vec::new();
        loop {
            let mut steps = 0;
            while self.layout.contains(&(self.bot.lookahead())) {
                self.bot.forward(1);
                steps += 1;
            }
            if steps > 0 {
                path.push(Instruction::Forward(steps));
            }
            if self.layout.contains(&(self.bot.look_left())) {
                self.bot.go(Instruction::TurnLeft);
                path.push(Instruction::TurnLeft);
            } else if self.layout.contains(&(self.bot.look_right())) {
                self.bot.go(Instruction::TurnRight);
                path.push(Instruction::TurnRight);
            } else {
                break;
            }
        }
        path
    }
}

fn compress(path: Vec<Instruction>) -> Vec<usize> {
    // find repeated pattern return mapping + copressed
    unimplemented!()
}

fn run(bot: &mut Computer, path: Vec<usize>) -> isize {
    for i in path {
        bot.input(i as isize).unwrap();
    }
    bot.output().unwrap()
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //find bounds
        let maxx = self.layout.iter().map(|&x| x.0).max().unwrap();
        let maxy = self.layout.iter().map(|&x| x.1).max().unwrap();

        for y in 0..=maxy {
            let line: Vec<String> = (0..=maxx)
                .map(|x| {
                    format!(
                        "{}",
                        if P(x, y) == self.bot.position {
                            self.bot.facing as u8 as char
                        } else if self.layout.contains(&P(x, y)) {
                            '#'
                        } else {
                            ' '
                        }
                    )
                })
                .collect();
            writeln!(f, "{}", line.join(""))?
        }
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("../inputs/day17.txt").unwrap();
    let program = Program::from(input);
    let computer = Computer::new(program.clone());

    let mut image = Image::read(computer);

    //println!("{}", image);

    // let part1 = image.calibration();
    // println!("{}", part1);

    let path = image.trace_path();
    let path = path
        .iter()
        .map(|x| match x {
            Instruction::TurnLeft => "L".to_string(),
            Instruction::TurnRight => "R".to_string(),
            Instruction::Forward(steps) => steps.to_string(),
        })
        .collect::<Vec<String>>()
        .join(",");

    println!("{}", path);
}

// A  -  R,8,R,12,L,8,
// B  -  L,8,R,12,L,8,
// C  -  R,10,R,10,
// D  -  L,8,R,12,R,12,R,10,L,10,

// A,B,C,D,C,D,C,B,D,B

// A - R,8,
// B - R,12,L,8,
// C - L,8,R,12,L,8,
// D - R,10,R,10,L,8,
// E - R,12,R,12,R,10,L,10,

// A,B,C,D,E,C,E,D,C,B,D,B
