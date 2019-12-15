extern crate intcode;
extern crate ncurses;

use intcode::{Computer, Program};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct P(isize, isize);

impl P {
    fn go(&self, direction: Move) -> P {
        match direction {
            Move::North => P(self.0, self.1 + 1),
            Move::South => P(self.0, self.1 - 1),
            Move::East => P(self.0 + 1, self.1),
            Move::West => P(self.0 - 1, self.1),
        }
    }

    fn adjacent(&self) -> Vec<P> {
        vec![
            self.go(Move::North),
            self.go(Move::South),
            self.go(Move::East),
            self.go(Move::West),
        ]
    }
}

#[derive(Clone, Copy)]
enum Move {
    North = 1,
    South = 2,
    East = 3,
    West = 4,
}

#[derive(Clone, Copy)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
    Bot,
    Unknown,
    Start,
}

impl Into<Tile> for isize {
    fn into(self) -> Tile {
        match self {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::Oxygen,
            t => panic!("unknown tile {}", t),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => "#",
                Tile::Empty => ".",
                Tile::Oxygen => "X",
                Tile::Bot => "O",
                Tile::Unknown => "?",
                Tile::Start => "*",
            }
        )
    }
}

struct Bot {
    cpu: Computer,
    position: P,
}

impl Bot {
    fn new(cpu: Computer) -> Self {
        Bot {
            cpu: cpu,
            position: P(0, 0),
        }
    }

    fn go(&mut self, direction: Move) -> Tile {
        self.cpu.input(direction as isize).unwrap();
        let tile = self.cpu.output().unwrap().into();

        self.position = match (tile, direction) {
            (Tile::Wall, _) => self.position,
            (_, direction) => self.position.go(direction),
        };
        tile
    }
}

struct Area {
    bot: Bot,
    layout: HashMap<P, Tile>,
}

impl Area {
    fn new(bot: Bot) -> Self {
        let mut layout = HashMap::new();
        layout.insert(P(0, 0), Tile::Start);
        Area {
            bot: bot,
            layout: layout,
        }
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //find bounds
        let minx = self.layout.keys().map(|&x| x.0).min().unwrap();
        let miny = self.layout.keys().map(|&x| x.1).min().unwrap();
        let maxx = self.layout.keys().map(|&x| x.0).max().unwrap();
        let maxy = self.layout.keys().map(|&x| x.1).max().unwrap();

        // reverse y because we're printing top-down
        for y in (miny..=maxy).rev() {
            let line: Vec<String> = (minx..=maxx)
                .map(|x| {
                    format!(
                        "{}",
                        if P(x, y) == P(0, 0) {
                            &Tile::Start
                        } else if P(x, y) == self.bot.position {
                            &Tile::Bot
                        } else {
                            self.layout.get(&P(x, y)).or(Some(&Tile::Unknown)).unwrap()
                        }
                    )
                })
                .collect();
            writeln!(f, "{}", line.join(""))?
        }
        Ok(())
    }
}

fn run_interactive(area: &mut Area) {
    ncurses::initscr();
    ncurses::keypad(ncurses::stdscr(), true);
    //ncurses::raw();
    ncurses::noecho();

    loop {
        ncurses::clear();

        ncurses::addstr(&format!("{}", &area));

        let direction = match ncurses::getch() {
            ncurses::constants::KEY_RIGHT => Move::East,
            ncurses::constants::KEY_LEFT => Move::West,
            ncurses::constants::KEY_UP => Move::North,
            ncurses::constants::KEY_DOWN => Move::South,
            _ => break,
        };
        let target = area.bot.position.go(direction);
        let tile = area.bot.go(direction);
        area.layout.insert(target, tile);

        ncurses::refresh();
    }

    ncurses::endwin();
    println!("{:?}", area.bot.position);
}

struct FloodFill {
    walls: HashSet<P>,
    distances: HashMap<P, usize>,
    queue: Vec<P>,
}

impl FloodFill {
    fn fill(&mut self) {
        while self.queue.len() > 0 {
            let current = self.queue.remove(0);
            // println!("{:?}", current);
            // println!("{:?}", self.distances);
            let adjacent = current.adjacent();
            // println!("{:?}", adjacent);
            let adjacent: Vec<P> = adjacent
                .iter()
                .filter(|x| !self.walls.contains(x))
                .map(|&x| x)
                .collect();
            // println!("{:?}", adjacent);
            let shortest = adjacent
                .iter()
                .filter(|x| self.distances.contains_key(x))
                .map(|x| *self.distances.get(x).unwrap())
                .min()
                .unwrap();
            self.distances.insert(current, shortest + 1);
            let mut adjacent: Vec<P> = adjacent
                .iter()
                .filter(|x| !self.queue.contains(x) && !self.distances.contains_key(x))
                .map(|&x| x)
                .collect();
            self.queue.append(&mut adjacent);
        }
    }
}

impl From<String> for FloodFill {
    fn from(input: String) -> Self {
        let mut start = P(0, 0);
        let mut walls = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = P(x as isize, y as isize);
                match c {
                    '#' => {
                        walls.insert(point);
                    }
                    'X' => start = point,
                    _ => (),
                };
            }
        }

        let mut distances = HashMap::new();
        distances.insert(start, 0);
        let queue: Vec<P> = start
            .adjacent()
            .iter()
            .filter(|x| !walls.contains(x))
            .map(|&x| x)
            .collect();

        FloodFill {
            walls: walls,
            distances: distances,
            queue: queue,
        }
    }
}

fn main() {
    // let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day15.txt").unwrap();
    // let program = Program::from(input);
    // let mut area = Area::new(Bot::new(Computer::new(program.clone())));

    //run_interactive(&mut area);

    let map = fs::read_to_string("/home/chris/advent_of_code/2019/day15/output.txt").unwrap();
    let mut flood = FloodFill::from(map);
    flood.fill();
    let part2 = flood.distances.values().max().unwrap();
    println!("{}", part2);
}
