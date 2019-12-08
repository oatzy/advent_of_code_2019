fn divmod(value: isize, quotient: isize) -> (isize, isize) {
    (value / quotient, value % quotient)
}

#[derive(Clone, Copy)]
enum Var {
    Immediate(isize),
    Positional(usize),
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
                _ => panic!("unexpected type {}", kind),
            })
        }
        out
    }
}

enum Op {
    Add(Var, Var, Var),      // 1
    Multiply(Var, Var, Var), // 2
    Input(Var),              // 3
    Output(Var),             // 4
    JumpIfTrue(Var, Var),    // 5
    JumpIfFalse(Var, Var),   // 6
    LessThan(Var, Var, Var), // 7
    Equal(Var, Var, Var),    // 8
    Halt,                    // 99
}

enum Interupt {
    Halt,
    Input(InputState),
    Output(OutputState),
}

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
            99 => Op::Halt,
            _ => panic!("got unexpected op code {}", op),
        }
    }

    fn get_var(&self, var: Var) -> isize {
        match var {
            Var::Immediate(value) => value,
            Var::Positional(position) => *self.memory.get(position).unwrap(),
        }
    }

    fn execute(mut self) -> Interupt {
        loop {
            match self.current() {
                Op::Add(left, right, out) => {
                    self.pointer += 4;
                    let out = self.get_var(out) as usize;
                    self.memory[out] = self.get_var(left) + self.get_var(right);
                }
                Op::Multiply(left, right, out) => {
                    self.pointer += 4;
                    let out = self.get_var(out) as usize;
                    self.memory[out] = self.get_var(left) * self.get_var(right);
                }
                Op::Input(position) => {
                    self.pointer += 2;
                    return Interupt::Input(InputState {
                        program: self,
                        position: match position {
                            Var::Positional(position) => position,
                            Var::Immediate(_) => panic!("expected positional var for input"),
                        },
                    });
                }
                Op::Output(value) => {
                    self.pointer += 2;
                    return Interupt::Output(OutputState {
                        program: self,
                        value: value,
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
                Op::LessThan(left, right, out) => {
                    self.pointer += 4;
                    let out = self.get_var(out) as usize;
                    self.memory[out] = if self.get_var(left) < self.get_var(right) {
                        1
                    } else {
                        0
                    };
                }
                Op::Equal(left, right, out) => {
                    self.pointer += 4;
                    let out = self.get_var(out) as usize;
                    self.memory[out] = if self.get_var(left) == self.get_var(right) {
                        1
                    } else {
                        0
                    };
                }
                Op::Halt => return Interupt::Halt,
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

struct InputState {
    program: Program,
    position: usize,
}

impl InputState {
    fn input(mut self, value: isize) -> Program {
        self.program.memory[self.position] = value;
        self.program
    }
}

struct OutputState {
    program: Program,
    value: Var,
}

impl OutputState {
    fn receive(self) -> (Program, isize) {
        let value = match self.value {
            Var::Immediate(value) => value,
            Var::Positional(pos) => *self.program.memory.get(pos).unwrap(),
        };
        (self.program, value)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
