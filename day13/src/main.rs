extern crate intcode;
// extern crate ncurses;

use intcode::{Computer, Interupt, Program};
use std::collections::HashMap;
use std::fmt;
use std::fs;

#[derive(Clone)]
enum State {
    NewGame(Program),
    Continue(Interupt),
    GameOver,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct P(isize, isize);

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
    Score(usize),
}

impl Tile {
    fn new(id: usize) -> Self {
        match id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            score => Tile::Score(score),
        }
    }

    fn to_string(&self) -> String {
        match self {
            Tile::Empty => " ".to_string(),
            Tile::Wall => "#".to_string(),
            Tile::Block => "X".to_string(),
            Tile::Paddle => "=".to_string(),
            Tile::Ball => "o".to_string(),
            Tile::Score(s) => s.to_string(),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

type Layout = HashMap<P, Tile>;

struct Game {
    program: State,
    layout: Layout,
    ballx: isize,
    paddlex: isize,
}

impl Game {
    fn new(program: Program) -> Self {
        Game {
            program: State::NewGame(program),
            layout: HashMap::new(),
            ballx: 0,
            paddlex: 0,
        }
    }

    fn input(mut self, joystick: isize) -> Self {
        if let State::Continue(Interupt::Input(input)) = self.program {
            let program = input.input(joystick);
            self.program = State::Continue(program.execute());
        } else {
            panic!("Can't input a non-input state");
        };
        self
    }

    fn refresh(mut self) -> Self {
        if let State::GameOver = self.program {
            return self;
        }

        //self.layout.clear();
        let mut outputs = Vec::new();
        loop {
            let prog = match self.program {
                State::GameOver => break,
                State::NewGame(program) => State::Continue(program.execute()),
                State::Continue(Interupt::Halt) => State::GameOver,
                State::Continue(Interupt::Input(_)) => break,
                State::Continue(Interupt::Output(output)) => {
                    let (program, value) = output.receive();
                    outputs.push(value);
                    if outputs.len() == 3 {
                        let pos = P(*outputs.get(0).unwrap(), *outputs.get(1).unwrap());
                        let tile = *outputs.get(2).unwrap() as usize;
                        let tile = if pos == P(-1, 0) {
                            Tile::Score(tile)
                        } else {
                            Tile::new(tile)
                        };
                        if tile == Tile::Ball {
                            self.ballx = pos.0;
                        }
                        if tile == Tile::Paddle {
                            self.paddlex = pos.0;
                        }
                        self.layout.insert(pos, tile);
                        outputs.clear();
                    }
                    State::Continue(program.execute())
                }
            };
            self.program = prog;
        }
        self
    }

    fn auto(mut self) -> usize {
        loop {
            self = self.refresh();
            if let State::GameOver = self.program {
                break;
            }
            let input = if self.ballx < self.paddlex {
                -1
            } else if self.ballx > self.paddlex {
                1
            } else {
                0
            };
            self = self.input(input);
        }
        if let Some(Tile::Score(score)) = self.layout.get(&P(-1, 0)) {
            return *score;
        }
        panic!("Score wasn't found");
    }

    // fn run(mut self) {
    //     ncurses::initscr();
    //     ncurses::keypad(ncurses::stdscr(), true);
    //     //ncurses::raw();
    //     ncurses::noecho();

    //     //let mut buf = String::new();
    //     for mut t in 0.. {
    //         ncurses::clear();
    //         self = self.refresh();
    //         if let State::GameOver = self.program {
    //             break;
    //         }
    //         //println!("{}", self);
    //         ncurses::addstr(&format!("{}", self));
    //         //io::stdin().read_line(&mut buf).unwrap();
    //         //self = self.input(buf.trim().parse().unwrap());
    //         //buf.clear();
    //         let input = match ncurses::getch() {
    //             ncurses::constants::KEY_RIGHT => 1,
    //             ncurses::constants::KEY_LEFT => -1,
    //             10 => 0, // enter
    //             _ => break,
    //         };
    //         self = self.input(input);
    //         ncurses::refresh();
    //     }
    //     ncurses::endwin();

    //     println!("{}", self);
    //     println!("GAME OVER!");
    // }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let width = self.layout.keys().map(|x| x.0).max().unwrap();
        let height = self.layout.keys().map(|x| x.1).max().unwrap();

        writeln!(
            f,
            "Current score: {}",
            self.layout
                .get(&P(-1, 0))
                .or(Some(&Tile::Score(0)))
                .unwrap()
        )?;

        for y in 0..=height {
            writeln!(
                f,
                "{}",
                (0..=width)
                    .map(|x| self
                        .layout
                        .get(&P(x as isize, y as isize))
                        .unwrap()
                        .to_string())
                    .collect::<Vec<String>>()
                    .join("")
            )?
        }
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("../inputs/day13.txt").unwrap();
    let mut program = Program::from(input);
    // let computer = Computer::new(program.clone());

    // let blocks = computer.skip(2).step_by(3).filter(|&x| x == 2).count();
    // println!("{}", blocks);

    program.set(0, 2);
    let game = Game::new(program);
    //game.run();
    let score = game.auto();
    println!("{}", score);
}
