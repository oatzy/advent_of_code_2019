fn divmod(value: isize, quotient: isize) -> (isize, isize) {
    (value / quotient, value % quotient)
}

#[derive(Debug, Clone, Copy)]
enum Var {
    Immediate(isize),
    Positional(usize),
    Relative(isize),
}

impl Var {
    fn parse(vars: &[isize], mask: isize) -> Vec<Var> {
        let mut mask = mask;
        let mut out = Vec::new();

        for i in vars.iter() {
            let (m, kind) = divmod(mask, 10);
            mask = m;
            out.push(match kind {
                0 => Var::Positional(*i as usize),
                1 => Var::Immediate(*i),
                2 => Var::Relative(*i),
                _ => panic!("unexpected type {}", kind),
            })
        }
        out
    }
}

#[derive(Debug)]
enum Op {
    Add(Var, Var, Var),      // 1
    Multiply(Var, Var, Var), // 2
    Input(Var),              // 3
    Output(Var),             // 4
    JumpIfTrue(Var, Var),    // 5
    JumpIfFalse(Var, Var),   // 6
    LessThan(Var, Var, Var), // 7
    Equal(Var, Var, Var),    // 8
    AdjustOffset(Var),       // 9
    Halt,                    // 99
}

pub enum Interupt {
    Halt,
    Input(InputState),
    Output(OutputState),
}

pub struct Program {
    memory: Vec<isize>,
    pointer: usize,
    offset: usize,
}

impl Program {
    fn new(input: Vec<isize>) -> Self {
        Program {
            memory: input,
            pointer: 0,
            offset: 0,
        }
    }

    fn current(&self) -> Op {
        let opcode = *self.memory.get(self.pointer).unwrap();
        let (mask, op) = divmod(opcode, 100);
        match op {
            1 => {
                let vars = Var::parse(&self.memory[self.pointer + 1..self.pointer + 4], mask);
                Op::Add(
                    *vars.get(0).unwrap(),
                    *vars.get(1).unwrap(),
                    *vars.get(2).unwrap(),
                )
            }
            2 => {
                let vars = Var::parse(&self.memory[self.pointer + 1..self.pointer + 4], mask);
                Op::Multiply(
                    *vars.get(0).unwrap(),
                    *vars.get(1).unwrap(),
                    *vars.get(2).unwrap(),
                )
            }
            3 => Op::Input(Var::Positional(
                *self.memory.get(self.pointer + 1).unwrap() as usize
            )),
            4 => Op::Output(
                *Var::parse(&self.memory[self.pointer + 1..self.pointer + 2], mask)
                    .get(0)
                    .unwrap(),
            ),
            5 => {
                let vars = Var::parse(&self.memory[self.pointer + 1..self.pointer + 3], mask);
                Op::JumpIfTrue(*vars.get(0).unwrap(), *vars.get(1).unwrap())
            }
            6 => {
                let vars = Var::parse(&self.memory[self.pointer + 1..self.pointer + 3], mask);
                Op::JumpIfFalse(*vars.get(0).unwrap(), *vars.get(1).unwrap())
            }
            7 => {
                let vars = Var::parse(&self.memory[self.pointer + 1..self.pointer + 4], mask);
                Op::LessThan(
                    *vars.get(0).unwrap(),
                    *vars.get(1).unwrap(),
                    *vars.get(2).unwrap(),
                )
            }
            8 => {
                let vars = Var::parse(&self.memory[self.pointer + 1..self.pointer + 4], mask);
                Op::Equal(
                    *vars.get(0).unwrap(),
                    *vars.get(1).unwrap(),
                    *vars.get(2).unwrap(),
                )
            }
            9 => Op::AdjustOffset(
                *Var::parse(&self.memory[self.pointer + 1..self.pointer + 2], mask)
                    .get(0)
                    .unwrap(),
            ),
            99 => Op::Halt,
            _ => panic!("got unexpected op code {}", op),
        }
    }

    fn get(&mut self, inx: usize) -> isize {
        if inx >= self.memory.len() {
            0
        } else {
            *self.memory.get(inx).unwrap()
        }
    }

    fn set(&mut self, inx: usize, value: isize) {
        if inx >= self.memory.len() {
            self.memory.resize(inx + 1, 0);
        }
        self.memory[inx] = value;
    }

    fn get_var(&mut self, var: Var) -> isize {
        match var {
            Var::Immediate(value) => value,
            Var::Positional(position) => self.get(position),
            Var::Relative(distance) => self.get((self.offset as isize + distance) as usize),
        }
    }

    pub fn execute(mut self) -> Interupt {
        loop {
            match self.current() {
                Op::Add(left, right, Var::Positional(out)) => {
                    //let out = self.get_var(out) as usize;
                    let value = self.get_var(left) + self.get_var(right);
                    self.set(out, value);
                    self.pointer += 4;
                }
                Op::Multiply(left, right, Var::Positional(out)) => {
                    //let out = self.get_var(out) as usize;
                    let value = self.get_var(left) * self.get_var(right);
                    self.set(out, value);
                    self.pointer += 4;
                }
                Op::Input(Var::Positional(position)) => {
                    //let out = self.get_var(position) as usize;
                    self.pointer += 2;
                    return Interupt::Input(InputState {
                        program: self,
                        position: position,
                    });
                }
                Op::Output(value) => {
                    let out = self.get_var(value);
                    self.pointer += 2;
                    return Interupt::Output(OutputState {
                        program: self,
                        value: out,
                    });
                }
                Op::JumpIfTrue(value, position) => {
                    if self.get_var(value) != 0 {
                        self.pointer = self.get_var(position) as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                Op::JumpIfFalse(value, position) => {
                    if self.get_var(value) == 0 {
                        self.pointer = self.get_var(position) as usize;
                    } else {
                        self.pointer += 3;
                    }
                }
                Op::LessThan(left, right, Var::Positional(out)) => {
                    //let out = self.get_var(out) as usize;
                    let value = if self.get_var(left) < self.get_var(right) {
                        1
                    } else {
                        0
                    };
                    self.set(out, value);
                    self.pointer += 4;
                }
                Op::Equal(left, right, Var::Positional(out)) => {
                    //let out = self.get_var(out) as usize;
                    let value = if self.get_var(left) == self.get_var(right) {
                        1
                    } else {
                        0
                    };
                    self.set(out, value);
                    self.pointer += 4;
                }
                Op::AdjustOffset(distance) => {
                    let dist = self.get_var(distance);
                    self.offset = (self.offset as isize + dist) as usize;
                    self.pointer += 2;
                }
                Op::Halt => return Interupt::Halt,
                _ => panic!("unexpected op code"),
            }
        }
    }
}

impl From<String> for Program {
    fn from(input: String) -> Self {
        Program::new(
            input
                .trim()
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect(),
        )
    }
}

pub struct InputState {
    program: Program,
    position: usize,
}

impl InputState {
    pub fn input(mut self, value: isize) -> Program {
        self.program.set(self.position, value);
        self.program
    }
}

pub struct OutputState {
    program: Program,
    value: isize,
}

impl OutputState {
    pub fn receive(self) -> (Program, isize) {
        (self.program, self.value)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_output_16digit_number() {
        use super::{Interupt, Program};

        let program = Program::new(vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);

        let result = match program.execute() {
            Interupt::Output(output) => output,
            _ => panic!("got unexpected interupt"),
        };
        let (program, value) = result.receive();
        println!("{:?}", program.memory);
        match program.execute() {
            Interupt::Halt => (),
            _ => panic!("got unexpected interupt"),
        };

        assert_eq!(value.to_string().len(), 16);
    }

    #[test]
    fn test_large_number() {
        use super::{Interupt, Program};

        let program = Program::new(vec![104, 1125899906842624, 99]);
        let result = match program.execute() {
            Interupt::Output(output) => output,
            _ => panic!("got unexpected interupt"),
        };
        let (program, value) = result.receive();
        match program.execute() {
            Interupt::Halt => (),
            _ => panic!("got unexpected interupt"),
        };

        assert_eq!(value, 1125899906842624);
    }

    #[test]
    fn test_quine() {
        use super::{Interupt, Program};

        let raw = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut outputs = Vec::new();

        let mut program = Program::new(raw.clone());

        while let Interupt::Output(output) = program.execute() {
            let (prog, value) = output.receive();
            outputs.push(value);
            program = prog;
        }

        assert_eq!(outputs, raw);
    }
}
