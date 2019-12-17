extern crate intcode;

use intcode::{Computer, Program};
use std::collections::HashMap;
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

struct Image {
    layout: HashMap<P, Tile>,
    bot: P,
}

impl Image {
    fn read(computer: Computer) -> Image {
        let mut layout = HashMap::new();
        let mut bot = P(0, 0);

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
                    layout.insert(P(x, y), Tile::Scaffold);
                }
                '.' => {
                    layout.insert(P(x, y), Tile::Empty);
                }
                '^' | 'v' | '<' | '>' => {
                    bot = P(x, y);
                    layout.insert(P(x, y), Tile::Scaffold);
                }
                c => panic!("unknown character {}", c),
            }
            x += 1;
        }

        Image {
            layout: layout,
            bot: bot,
        }
    }

    fn is_intersection(&self, pos: &P) -> bool {
        pos.adjacent()
            .iter()
            .chain(&[*pos])
            .all(|x| self.layout.get(x).or(Some(&Tile::Empty)).unwrap() == &Tile::Scaffold)
    }

    fn calibration(&self) -> isize {
        self.layout
            .keys()
            .filter(|&x| self.is_intersection(x))
            .map(|x| x.0 * x.1)
            .sum()
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //find bounds
        let maxx = self.layout.keys().map(|&x| x.0).max().unwrap();
        let maxy = self.layout.keys().map(|&x| x.1).max().unwrap();

        for y in 0..=maxy {
            let line: Vec<String> = (0..=maxx)
                .map(|x| {
                    format!(
                        "{}",
                        if P(x, y) == self.bot {
                            'O'
                        } else {
                            // this is fun
                            *self.layout.get(&P(x, y)).or(Some(&Tile::Empty)).unwrap() as u8 as char
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
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day17.txt").unwrap();
    let program = Program::from(input);
    let computer = Computer::new(program.clone());

    let image = Image::read(computer);

    //println!("{}", image);

    let part1 = image.calibration();
    println!("{}", part1);
}
