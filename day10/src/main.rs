use std::collections::{HashMap, HashSet};
use std::f32;
use std::fs;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Point(isize, isize);

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
                '#' => points.push(Point(x as isize, y as isize)),
                '.' => (),
                c => panic!("unexpected char {}", c),
            };
        }
    }
    points
}

fn canonical(source: Point, target: Point) -> Angle {
    // get the position of a point relative to the source
    // then calculate a 'canonical' form
    // all points with the same canonical form lie on a line from source
    let x = target.0 as isize - source.0 as isize;
    let y = target.1 as isize - source.1 as isize;
    let g = gcd(x, y);

    Angle(x / g, y / g)
}

fn viewable_from(origin: Point, points: &Vec<Point>) -> usize {
    points
        .iter()
        .filter(|&x| x != &origin) // skip the point itself
        .map(|&x| canonical(origin, x))
        .collect::<HashSet<Angle>>()
        .len()
}

fn find_optimal_position(points: &Vec<Point>) -> (usize, Point) {
    points
        .iter()
        .map(|&x| (viewable_from(x, points), x))
        .max()
        .unwrap()
}

fn angle(x: isize, y: isize) -> f32 {
    let y = -y; // our y axis is flipped
    let mut a = f32::atan2(x as f32, y as f32);
    if a < 0_f32 {
        // shift -ive x to be after +ive x
        a += 2_f32 * f32::consts::PI;
    }
    a
}

fn sort_points(origin: Point, points: &Vec<Point>) -> Vec<Point> {
    let mut angles = HashMap::new();
    let mut output = Vec::new();

    // build a map of {canonical: [point]}
    for point in points.iter().filter(|&x| x != &origin) {
        angles
            .entry(canonical(origin, *point))
            .or_insert(vec![])
            .push(point);
    }

    for values in angles.values() {
        // sort points with the same canonical by distance from origin
        let mut values: Vec<(isize, &Point)> = values
            .iter()
            .map(|&x| ((origin.0 - x.0) ^ 2 + (origin.1 - x.1) ^ 2, x))
            .collect();
        values.sort();

        // add multiples of 2 pi to each angle according to distance
        // essentially the first is angle A, second is angle A + 1 rotation
        // third is angle A + 2 rotations, etc
        // this means we can sort by angle and points which are 'obscured'
        // in the first rotation will appear in the next rotation
        for (inx, value) in values.iter().map(|(_, &x)| x).enumerate() {
            output.push((
                angle(value.0 - origin.0, value.1 - origin.1)
                    + (inx as f32 * 2_f32 * f32::consts::PI),
                value,
            ));
        }
    }

    // sort points by the 'adjusted' angle
    output.sort_by(|a, b| a.partial_cmp(b).unwrap());
    output.iter().map(|(_, x)| *x).collect()
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day10.txt").unwrap();
    let points = parse(input.trim().to_owned());

    let (viewable, optimal) = find_optimal_position(&points);
    println!("{}", viewable);

    let sorted = sort_points(optimal, &points);
    let point200 = sorted.get(199).unwrap();
    let part2 = 100 * point200.0 + point200.1;
    println!("{:?}", part2);
}
