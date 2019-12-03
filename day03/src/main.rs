use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::fs;
use std::iter;
use std::ops::Add;
use std::usize::MAX;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct Point(isize, isize);

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

impl Point {
    fn distance(&self, other: &Self) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn make_repeater(instruction: &str) -> std::iter::Take<std::iter::Repeat<Point>> {
    iter::repeat(match instruction.chars().next().unwrap() {
        'U' => Point(0, 1),
        'D' => Point(0, -1),
        'L' => Point(-1, 0),
        'R' => Point(1, 0),
        _ => panic!("unexpected direction"),
    })
    .take(instruction[1..].parse().unwrap())
}

type Wire<'a> = Vec<&'a str>;

fn closest_crossing(wire1: &Wire, wire2: &Wire) -> usize {
    let mut points: HashSet<Point> = HashSet::new();
    let mut closest: usize = MAX;

    let mut current = Point(0, 0);

    for shift in wire1.iter().map(|&x| make_repeater(x)).flatten() {
        current = current + shift;
        points.insert(current);
    }

    let mut current = Point(0, 0);

    for shift in wire2.iter().map(|&x| make_repeater(x)).flatten() {
        current = current + shift;
        if points.contains(&current) {
            closest = closest.min(current.distance(&Point(0, 0)) as usize);
        }
    }

    closest
}

fn closest_step_crossing(wire1: &Wire, wire2: &Wire) -> usize {
    let mut points: HashMap<Point, usize> = HashMap::new();
    let mut closest: usize = MAX;

    let mut current = Point(0, 0);

    for (step, shift) in wire1
        .iter()
        .map(|&x| make_repeater(x))
        .flatten()
        .enumerate()
    {
        current = current + shift;
        points.entry(current).or_insert(step + 1);
    }

    let mut current = Point(0, 0);

    for (step, shift) in wire2
        .iter()
        .map(|&x| make_repeater(x))
        .flatten()
        .enumerate()
    {
        current = current + shift;
        if let Some(distance) = points.get(&current) {
            closest = closest.min(step + 1 + distance);
        }
    }

    closest
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day03.txt").unwrap();
    let mut input = input.lines();

    let wire1: Wire = input.next().unwrap().split(",").collect();
    let wire2: Wire = input.next().unwrap().split(",").collect();

    let part1 = closest_crossing(&wire1, &wire2);
    println!("{}", part1);

    let part2 = closest_step_crossing(&wire1, &wire2);
    println!("{}", part2);
}

mod test {

    #[test]
    fn test_closest_crossing() {
        use super::closest_crossing;

        let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(",").collect();
        let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83".split(",").collect();
        assert_eq!(closest_crossing(&wire1, &wire2), 159);

        let wire1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .split(",")
            .collect();
        let wire2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(",").collect();
        assert_eq!(closest_crossing(&wire1, &wire2), 135);
    }

    #[test]
    fn test_closest_step_crossing() {
        use super::closest_step_crossing;

        let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72".split(",").collect();
        let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83".split(",").collect();
        assert_eq!(closest_step_crossing(&wire1, &wire2), 610);

        let wire1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"
            .split(",")
            .collect();
        let wire2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".split(",").collect();
        assert_eq!(closest_step_crossing(&wire1, &wire2), 410);
    }
}
