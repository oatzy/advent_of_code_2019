use std::collections::HashMap;
use std::fs;

type Orbits<'a> = HashMap<&'a str, &'a str>;

fn parse_orbits(input: &str) -> Orbits {
    let mut orbits = Orbits::new();

    for line in input.lines() {
        let centre = &line[..3];
        let orbiter = &line[4..];
        orbits.entry(orbiter).or_insert(centre);
    }
    orbits
}

fn count_orbits(orbits: &Orbits, id: &str) -> usize {
    if id == "COM" {
        0
    } else {
        count_orbits(orbits, orbits.get(id).unwrap()) + 1
    }
}

fn count_all_orbits(orbits: &Orbits) -> usize {
    orbits.keys().map(|x| count_orbits(&orbits, x)).sum()
}

fn shortest_path(orbits: &Orbits) -> usize {
    // first we build a map of nodes to distance
    // going from YOU to COM
    // then we trace a path from SAN
    // until we reach a node that as in the YOU PATH
    let mut path = HashMap::<&str, usize>::new();
    let mut distance: usize = 0;

    let mut current = &"YOU";

    while current != &"COM" {
        current = orbits.get(current).unwrap();
        path.insert(current, distance);
        distance += 1;
    }

    current = &"SAN";
    distance = 0;

    while !path.contains_key(current) {
        current = orbits.get(current).unwrap();
        distance += 1;
    }

    distance + path.get(current).unwrap() - 1
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day06.txt").unwrap();

    let orbits = parse_orbits(&input);
    let part1: usize = count_all_orbits(&orbits);
    let part2: usize = shortest_path(&orbits);

    println!("{}", part1);
    println!("{}", part2);
}

mod test {

    #[test]
    fn test_count_all_orbits() {
        use super::{count_all_orbits, parse_orbits};

        let input = "COM)BBB
BBB)CCC
CCC)DDD
DDD)EEE
EEE)FFF
BBB)GGG
GGG)HHH
DDD)III
EEE)JJJ
JJJ)KKK
KKK)LLL";
        let orbits = parse_orbits(&input);
        assert_eq!(count_all_orbits(&orbits), 42);
    }

    #[test]
    fn test_shortest_path() {
        use super::{parse_orbits, shortest_path};

        let input = "COM)BBB
BBB)CCC
CCC)DDD
DDD)EEE
EEE)FFF
BBB)GGG
GGG)HHH
DDD)III
EEE)JJJ
JJJ)KKK
KKK)LLL
KKK)YOU
III)SAN";
        let orbits = parse_orbits(&input);
        assert_eq!(shortest_path(&orbits), 4);
    }
}
