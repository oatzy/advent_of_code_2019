use std::fs;
use std::iter;

type V = Vec<isize>;

fn FFT(signal: &V, pattern: &V) -> V {
    signal
        .iter()
        .enumerate()
        .map(|(i, _)| {
            signal
                .iter()
                .zip(
                    pattern
                        .iter()
                        .map(|x| iter::repeat(x).take(i + 1))
                        .flatten()
                        .cycle()
                        .skip(1),
                )
                .map(|(a, b)| (a * b))
                .sum()
        })
        .map(|x: isize| x.abs() % 10)
        .collect()
}

fn fft_phases(signal: &V, pattern: &V, loops: usize) -> V {
    let mut output = signal.clone();
    for _ in 0..loops {
        output = FFT(&output, &pattern);
    }
    output
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day16.txt").unwrap();
    let input: V = input
        .trim()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as isize)
        .collect();
    let pattern = vec![0, 1, 0, -1];

    let part1 = fft_phases(&input, &pattern, 100)
        .iter()
        .take(8)
        .map(|&x| x)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("{}", part1);
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
