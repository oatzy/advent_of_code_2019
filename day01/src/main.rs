use std::fs;

fn fuel(module: isize) -> isize {
    let mut total = 0;
    let mut current = (module / 3) - 2;

    while current > 0 {
        total = total + current;
        current = (current / 3) - 2;
    }
    total
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day01.txt").unwrap();
    let input: Vec<isize> = input.lines().map(|x| x.parse().unwrap()).collect();

    let part1: isize = input.iter().map(|x| (x / 3) - 2).sum();
    let part2: isize = input.iter().map(|&x| fuel(x)).sum();

    println!("{}", part1);
    println!("{}", part2);
}

mod test {

    #[test]
    fn test_fuel() {
        use super::fuel;

        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 966);
        assert_eq!(fuel(100756), 50346);
    }
}
