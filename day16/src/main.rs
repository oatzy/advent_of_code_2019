use std::fs;
use std::iter;

type V = Vec<isize>;

fn fft_digit(signal: &V, pattern: &V, digit: usize) -> isize {
    signal
        .iter()
        .zip(
            pattern
                .iter()
                .map(|x| iter::repeat(x).take(digit + 1))
                .flatten()
                .cycle()
                .skip(1),
        )
        .skip(digit)
        .map(|(a, b)| (a * b))
        .sum::<isize>()
        .abs()
        % 10
}

fn fft_digit_fn(signal: &V, digit: usize) -> isize {
    // slightly faster than the nested iterators
    signal
        .iter()
        .enumerate()
        .map(|(n, &x)| match ((n + 1) / (digit + 1)) % 4 {
            1 => x,
            3 => -x,
            _ => 0,
        })
        .sum::<isize>()
        .abs()
        % 10
}

fn FFT(signal: &V, pattern: &V) -> V {
    (0..signal.len())
        //.map(|i| fft_digit(&signal, &pattern, i))
        .map(|i| fft_digit_fn(&signal, i))
        .collect()
}

fn fft_phases(signal: &V, pattern: &V, loops: usize) -> V {
    let mut output = signal.clone();
    for i in 0..loops {
        output = FFT(&output, &pattern);
        println!("{}", i);
    }
    output
}

fn part2(input: &String) -> V {
    let start: usize = input[..7].parse().unwrap();
    let target_length = 10_000 * input.trim().len();

    // println!("{} - {}", start, target_length);

    let mut input: V = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as isize)
        .cycle()
        .skip(start)
        .take(target_length - start)
        .collect();

    for i in 0..100 {
        println!("loop {}", i);

        input.reverse();
        let mut transformed: V = input
            .iter()
            .scan(0, |acc, &x| {
                *acc = (*acc + x) % 10;
                Some(*acc)
            })
            .collect();
        transformed.reverse();
        input = transformed;
    }
    input
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day16.txt").unwrap();

    // let pattern = vec![0, 1, 0, -1];

    // let input: V = input
    //     .trim()
    //     .chars()
    //     .map(|x| x.to_digit(10).unwrap() as isize)
    //     .collect();

    // let part1 = fft_phases(&input, &pattern, 100)
    //     .iter()
    //     .take(8)
    //     .map(|&x| x)
    //     .map(|x| x.to_string())
    //     .collect::<Vec<String>>()
    //     .join("");

    // println!("{}", part1);

    let part2 = part2(&input)
        .iter()
        .take(8)
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("{}", part2);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_example() {
        use super::FFT;

        let pattern = vec![0, 1, 0, -1];
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(FFT(&signal, &pattern), vec![4, 8, 2, 2, 6, 1, 5, 8]);
    }

    #[test]
    fn test_example_one_loop() {
        use super::fft_phases;

        let pattern = vec![0, 1, 0, -1];
        let signal = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(
            fft_phases(&signal, &pattern, 1),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        );
    }

    #[test]
    fn test_loops() {
        use super::fft_phases;

        let pattern = vec![0, 1, 0, -1];
        let signal = vec![
            8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8, 6, 4, 5,
            5, 9, 5,
        ];

        assert_eq!(
            fft_phases(&signal, &pattern, 100)
                .iter()
                .take(8)
                .map(|&x| x)
                .collect::<Vec<isize>>(),
            vec![2, 4, 1, 7, 6, 1, 7, 6]
        )
    }
}
