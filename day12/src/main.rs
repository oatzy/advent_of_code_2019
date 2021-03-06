extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::ops::Add;

fn cmp(left: isize, right: isize) -> isize {
    match left.cmp(&right) {
        Ordering::Equal => 0,
        Ordering::Less => 1,
        Ordering::Greater => -1,
    }
}

#[derive(Clone, Copy)]
struct Triple {
    x: isize,
    y: isize,
    z: isize,
}

impl Triple {
    fn energy(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn gravity(&self, other: Self) -> Self {
        Triple {
            x: cmp(self.x, other.x),
            y: cmp(self.y, other.y),
            z: cmp(self.z, other.z),
        }
    }
}

impl Add for Triple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Triple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

type P = Triple;
type V = Triple;

#[derive(Clone, Copy)]
struct Moon {
    position: P,
    velocity: V,
}

impl Moon {
    fn new(position: P) -> Self {
        Moon {
            position: position,
            velocity: V { x: 0, y: 0, z: 0 },
        }
    }

    fn step(&mut self, gravity: Triple) {
        self.velocity = self.velocity + gravity;
        self.position = self.position + self.velocity;
    }

    fn gravity(&self, moons: &Vec<Moon>) -> V {
        moons
            .iter()
            .map(|x| self.position.gravity(x.position))
            .fold(V { x: 0, y: 0, z: 0 }, |x, y| x + y)
    }

    fn potential(&self) -> isize {
        self.position.energy()
    }

    fn kinetic(&self) -> isize {
        self.velocity.energy()
    }

    fn total_energy(&self) -> isize {
        self.kinetic() * self.potential()
    }
}

struct Moons {
    moons: Vec<Moon>
}

impl Moons {
    fn new(moons: Vec<Moon>) -> Self {
        Moons {
            moons: moons
        }
    }

    fn step(&mut self) {
        let gravities: Vec<V> = self.moons.iter().map(|m| m.gravity(&self.moons)).collect();

        for (m, g) in self.moons.iter_mut().zip(gravities) {
            m.step(g);
        }
    }

    fn total_energy(&self) -> isize {
        self.moons.iter().map(|m| m.total_energy()).sum()
    }
}

impl fmt::Debug for Moons {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for m in &self.moons {
            write!(f, "<x={}, y={}, z={}> ", m.position.x, m.position.y, m.position.z)?
        };
        Ok(())
    }
}

fn total_energy(moons: &mut Moons, iterations: usize) -> isize {
    for _ in 0..iterations {
        moons.step();
    }
    moons.total_energy()
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day12.txt").unwrap();

    let re = Regex::new(r"<x=(\-?\d+), y=(\-?\d+), z=(\-?\d+)>").unwrap();
    let moons: Vec<Moon> = re
        .captures_iter(&input)
        .map(|c| {
            Moon::new(P {
                x: c[1].parse().unwrap(),
                y: c[2].parse().unwrap(),
                z: c[3].parse().unwrap(),
            })
        })
        .collect();
    let mut moons = Moons::new(moons);

    //let part1 = total_energy(&mut moons, 1000);
    //println!("{}", part1);

    loop {
        moons.step();
        let kinetic: isize = moons.moons.iter().map(|m| m.kinetic()).sum();
        let potential: isize = moons.moons.iter().map(|m| m.potential()).sum();
        println!("{} {}", kinetic, potential);
    }
}
