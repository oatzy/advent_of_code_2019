use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Point(usize, usize);

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Angle(isize, isize);

fn gcd(m: isize, n: isize) -> isize {
    // https://rosettacode.org/wiki/Greatest_common_divisor#Rust
    let mut m = m;
    let mut n = n;

    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}

fn parse(input: String) -> Vec<Point> {
    let mut points = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => points.push(Point(x, y)),
                '.' => (),
                c => panic!("unexpected char {}", c),
            };
        }
    }
    points
}

fn angle(source: Point, target: Point) -> Angle {
    let x = target.0 as isize - source.0 as isize;
    let y = target.1 as isize - source.1 as isize;
    let g = gcd(x, y);

    Angle(x / g, y / g)
}

fn viewable_from(origin: Point, points: &Vec<Point>) -> usize {
    points
        .iter()
        .filter(|&x| x != &origin) // skip the point itself
        .map(|&x| angle(origin, x))
        .collect::<HashSet<Angle>>()
        .len()
}

fn find_most_viewable(points: &Vec<Point>) -> usize {
    points
        .iter()
        .map(|&x| viewable_from(x, points))
        .max()
        .unwrap()
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day10.txt").unwrap();
    let points = parse(input.trim().to_owned());

    let part1 = find_most_viewable(&points);
    println!("{}", part1);
}

mod test {

    #[test]
    fn test_part1() {
        unimplemented!()
    }
}
