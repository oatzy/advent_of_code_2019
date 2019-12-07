use std::fs;

mod permutations;
use permutations::permutations;

fn divmod(value: isize, quotient: isize) -> (isize, isize) {
    (value / quotient, value % quotient)
}

enum Interupt {
    Halt,
    Output(isize),
    Input(usize),
}

#[derive(Clone)]
struct Program {
    memory: Vec<isize>,
    pointer: usize,
}

impl Program {
    fn new(input: Vec<isize>) -> Self {
        Program {
            memory: input,
            pointer: 0,
        }
    }

    fn from_string(input: String) -> Self {
        Program::new(
            input
                .trim()
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect(),
        )
    }

    fn run(&mut self) -> Interupt {
        while let Some(&opcode) = self.memory.get(self.pointer) {
            let (types, op) = divmod(opcode, 100);

            if op == 99 {
                // terminate opcode
                break;
            } else if op == 3 {
                // get input
                let &outp = self.memory.get(self.pointer + 1).unwrap();
                self.pointer += 2;
                return Interupt::Input(outp as usize);
            } else if op == 4 {
                // return output
                let &inp = self.memory.get(self.pointer + 1).unwrap();
                self.pointer += 2;
                return Interupt::Output(match divmod(types, 10) {
                    (_, 0) => *self.memory.get(inp as usize).unwrap(),
                    (_, 1) => *&inp,
                    _ => panic!("got unexpected type"),
                });
            } else if op == 5 || op == 6 {
                // jump if true/false
                let (types, type1) = divmod(types, 10);
                let (_, type2) = divmod(types, 10);

                let mut inp = self.memory.get(self.pointer + 1).unwrap();
                if type1 == 0 {
                    inp = self.memory.get(*inp as usize).unwrap();
                };
                let mut outp = self.memory.get(self.pointer + 2).unwrap();
                if type2 == 0 {
                    outp = self.memory.get(*outp as usize).unwrap();
                };

                if (op == 5 && *inp != 0) || (op == 6 && *inp == 0) {
                    self.pointer = *outp as usize;
                } else {
                    self.pointer += 3;
                }
            } else {
                let (types, type1) = divmod(types, 10);
                let (_, type2) = divmod(types, 10);

                let mut inp1 = self.memory.get(self.pointer + 1).unwrap();
                if type1 == 0 {
                    inp1 = self.memory.get(*inp1 as usize).unwrap();
                };
                let mut inp2 = self.memory.get(self.pointer + 2).unwrap();
                if type2 == 0 {
                    inp2 = self.memory.get(*inp2 as usize).unwrap();
                };
                let &outp = self.memory.get(self.pointer + 3).unwrap();

                self.memory[outp as usize] = match op {
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

                self.pointer += 4;
            }
        }
        Interupt::Halt
    }

    fn input(&mut self, position: usize, value: isize) {
        self.memory[position] = value;
    }
}

fn run_intcode(program: &Vec<isize>, inputs: &Vec<isize>) -> Vec<isize> {
    let mut program = Program::new(program.clone());
    let mut ininx = 0;
    let mut output = Vec::new();

    loop {
        match program.run() {
            Interupt::Halt => break,
            Interupt::Input(position) => {
                let &inval = inputs.get(ininx).unwrap();
                ininx += 1;
                program.input(position, inval);
            }
            Interupt::Output(value) => {
                output.push(value);
            }
        };
    }

    output
}

fn thruster_signal(program: &Vec<isize>, config: &Vec<usize>) -> isize {
    let mut out = 0;
    for i in 0..5 {
        out = *run_intcode(program, &vec![*config.get(i).unwrap() as isize, out])
            .get(0)
            .unwrap();
    }
    out
}

fn max_output(program: &Vec<isize>) -> isize {
    let mut maxout = 0;

    for perm in permutations(5) {
        let out = thruster_signal(program, &perm);
        if out > maxout {
            maxout = out;
        };
    }

    maxout
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day07.txt").unwrap();
    let input: Vec<isize> = input
        .trim()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    let part1 = max_output(&input);
    println!("{}", part1);
}

mod test {

    #[test]
    fn test_run_intcode() {
        use super::max_output;

        assert_eq!(
            max_output(&vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ]),
            43210
        );
    }
}
