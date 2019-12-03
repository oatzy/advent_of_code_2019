use std::fs;

fn run_intcode(input: &Vec<usize>) -> Vec<usize> {
    let mut output = input.clone();
    let mut inx: usize = 0;

    while let Some(&op) = output.get(inx) {
        if op == 99 {  // terminate opcode
            break;
        };

        let &inp1 = output.get(inx + 1).unwrap();
        let &inp2 = output.get(inx + 2).unwrap();
        let &outp = output.get(inx + 3).unwrap();

        output[outp] = match op {
            1 => output.get(inp1).unwrap() + output.get(inp2).unwrap(),
            2 => output.get(inp1).unwrap() * output.get(inp2).unwrap(),
            _ => panic!("got unexpected op code {}", op),
        };

        inx += 4;
    }
    output
}

fn mod_run(input: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut input = input.clone();
    input[1] = noun;
    input[2] = verb;
    let output = run_intcode(&input);
    *output.get(0).unwrap()
}

fn part2(input: &Vec<usize>, target: usize) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            if mod_run(input, noun, verb) == target {
                return 100 * noun + verb;
            }
        }
    }
    panic!("a result wasn't found");
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day02.txt").unwrap();
    let input: Vec<usize> = input
        .trim_end()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let part1 = mod_run(&input, 12, 2);
    println!("{}", part1);

    let part2 = part2(&input, 19690720);
    println!("{}", part2);
}

mod test {

    #[test]
    fn test_run_intcode() {
        use super::run_intcode;

        assert_eq!(run_intcode(&vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
        assert_eq!(run_intcode(&vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
        assert_eq!(
            run_intcode(&vec![2, 4, 4, 5, 99, 0]),
            vec![2, 4, 4, 5, 99, 9801]
        );
        assert_eq!(
            run_intcode(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
