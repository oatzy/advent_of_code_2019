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
        // set value at position
        self.memory[position] = value;
    }

    fn call(&mut self, input: isize) -> Option<isize> {
        // run until input is requested (or halt)
        // provide input, then run until output or halt
        match self.run() {
            Interupt::Input(position) => self.input(position, input),
            Interupt::Halt => return None,
            _ => panic!("got a non-input interupt before request for input"),
        };
        match self.run() {
            Interupt::Halt => None,
            Interupt::Output(value) => Some(value),
            Interupt::Input(_) => panic!("got unexpected request for input"),
        }
    }

    fn configure(&mut self, setting: isize) {
        match self.run() {
            Interupt::Input(position) => self.input(position, setting),
            _ => panic!("got a non-input interupt before program was configured"),
        };
    }
}

fn run_intcode(mut program: Program, inputs: &Vec<isize>) -> Vec<isize> {
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

fn thruster_signal(program: &Program, config: &Vec<usize>) -> isize {
    let mut out = 0;
    for i in 0..5 {
        let program = program.clone();
        out = *run_intcode(program, &vec![*config.get(i).unwrap() as isize, out])
            .get(0)
            .unwrap();
    }
    out
}

fn looped_thruster_signal(program: &Program, config: &Vec<usize>) -> isize {
    // storing the amps in a Vec would be much neater
    // but I don't know what to do about 'can't borrow as mutable'
    let mut out = 0;

    // create amplifiers and configure them
    let mut amp1 = program.clone();
    amp1.configure(*config.get(0).unwrap() as isize);
    let mut amp2 = program.clone();
    amp2.configure(*config.get(1).unwrap() as isize);
    let mut amp3 = program.clone();
    amp3.configure(*config.get(2).unwrap() as isize);
    let mut amp4 = program.clone();
    amp4.configure(*config.get(3).unwrap() as isize);
    let mut amp5 = program.clone();
    amp5.configure(*config.get(4).unwrap() as isize);

    loop {
        match amp1.call(out) {
            None => break,
            Some(value) => out = value,
        };
        match amp2.call(out) {
            None => break,
            Some(value) => out = value,
        };
        match amp3.call(out) {
            None => break,
            Some(value) => out = value,
        };
        match amp4.call(out) {
            None => break,
            Some(value) => out = value,
        };
        match amp5.call(out) {
            None => break,
            Some(value) => out = value,
        };
    }

    out
}

fn max_output(program: &Program) -> isize {
    let mut maxout = 0;

    for perm in permutations(0, 5) {
        let out = thruster_signal(program, &perm);
        if out > maxout {
            maxout = out;
        };
    }

    maxout
}

fn max_loop_output(program: &Program) -> isize {
    let mut maxout = 0;

    for perm in permutations(5, 10) {
        let out = looped_thruster_signal(program, &perm);
        if out > maxout {
            maxout = out;
        };
    }

    maxout
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day07.txt").unwrap();
    let program = Program::from_string(input);

    let part1 = max_output(&program);
    println!("{}", part1);

    let part2 = max_loop_output(&program);
    println!("{}", part2);
}

mod test {

    #[test]
    fn test_max_output() {
        use super::{max_output, Program};

        assert_eq!(
            max_output(&Program::new(vec![
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            ])),
            43210
        );

        assert_eq!(
            max_output(&Program::new(vec![
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            ])),
            54321
        );

        assert_eq!(
            max_output(&Program::new(vec![
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            ])),
            65210
        );
    }

    #[test]
    fn test_max_looped_output() {
        use super::{max_loop_output, Program};

        assert_eq!(
            max_loop_output(&Program::new(vec![
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5
            ])),
            139629729
        );

        assert_eq!(
            max_loop_output(&Program::new(vec![
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
            ])),
            18216
        );
    }
}
