use std::fs;
use std::io;

fn divmod(value: isize, quotient: isize) -> (isize, isize) {
    (value / quotient, value % quotient)
}

fn run_intcode(input: &Vec<isize>) -> Vec<isize> {
    let mut output = input.clone();
    let mut inx: usize = 0;
    let mut inbuf = String::new();

    while let Some(&opcode) = output.get(inx) {
        let (types, op) = divmod(opcode, 100);

        if op == 99 {
            // terminate opcode
            break;
        } else if op == 3 {
            io::stdin().read_line(&mut inbuf).unwrap();
            let inval: isize = inbuf.trim().parse().unwrap();
            let &outp = output.get(inx + 1).unwrap();
            output[outp as usize] = inval;
            inx += 2;
        } else if op == 4 {
            let &inp = output.get(inx + 1).unwrap();
            println!(
                "{}",
                match divmod(types, 10) {
                    (_, 0) => output.get(inp as usize).unwrap(),
                    (_, 1) => &inp,
                    _ => panic!("got unexpected type"),
                }
            );
            inx += 2;
        } else if op == 5 || op == 6 {
            let (types, type1) = divmod(types, 10);
            let (_, type2) = divmod(types, 10);

            let mut inp = output.get(inx + 1).unwrap();
            if type1 == 0 {
                inp = output.get(*inp as usize).unwrap();
            };
            let mut outp = output.get(inx + 2).unwrap();
            if type2 == 0 {
                outp = output.get(*outp as usize).unwrap();
            };

            if (op == 5 && *inp != 0) || (op == 6 && *inp == 0) {
                inx = *outp as usize;
            } else {
                inx += 3;
            }
        } else {
            let (types, type1) = divmod(types, 10);
            let (_, type2) = divmod(types, 10);

            let mut inp1 = output.get(inx + 1).unwrap();
            if type1 == 0 {
                inp1 = output.get(*inp1 as usize).unwrap();
            };
            let mut inp2 = output.get(inx + 2).unwrap();
            if type2 == 0 {
                inp2 = output.get(*inp2 as usize).unwrap();
            };
            let &outp = output.get(inx + 3).unwrap();

            output[outp as usize] = match op {
                1 => inp1 + inp2,
                2 => inp1 * inp2,
                7 => {
                    if inp1 < inp2 {
                        1
                    } else {
                        0
                    }
                }
                8 => {
                    if inp1 == inp2 {
                        1
                    } else {
                        0
                    }
                }
                _ => panic!("got unexpected op code {}", op),
            };

            inx += 4;
        }
    }
    output
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day05.txt").unwrap();
    let input: Vec<isize> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    run_intcode(&input);
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
