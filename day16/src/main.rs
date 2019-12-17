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

fn fft_from(signal: &V, pattern: &V, start: usize) -> V {
    (0..signal.len())
        .map(|i| fft_digit_fn(&signal, i + start))
        .collect()
}

fn fft_phases_from(signal: &V, pattern: &V, loops: usize, start: usize) -> V {
    println!("cloning input");
    let mut output = signal.clone();
    println!("looping");
    for i in 0..loops {
        output = fft_from(&output, &pattern, start);
        println!("{}", i);
    }
    output
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day16.txt").unwrap();

    let pattern = vec![0, 1, 0, -1];

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

    let start: usize = input[..7].parse().unwrap();
    let target_length = 10_000 * input.len();

    println!("{} - {}", start, target_length);

    println!("building input");
    let input: V = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as isize)
        .cycle()
        .skip(start)
        .take(target_length - start)
        .collect();

    println!("performing transform");
    let part2 = fft_phases_from(&input, &pattern, 100, start)
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
