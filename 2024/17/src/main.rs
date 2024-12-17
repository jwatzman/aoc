use std::env;
use std::fs;

type Register = u64;
type Instruction = u8;

#[derive(Debug)]
struct Computer {
    a: Register,
    b: Register,
    c: Register,
    ip: usize,
    instructions: Vec<Instruction>,
    out: Vec<Register>,
}

impl Computer {
    fn parse_input_register(s: &str) -> Register {
        s.split(':').last().unwrap().trim().parse().unwrap()
    }

    fn new(contents: String) -> Self {
        let mut it = contents.split('\n');

        let a = Self::parse_input_register(it.next().unwrap());
        let b = Self::parse_input_register(it.next().unwrap());
        let c = Self::parse_input_register(it.next().unwrap());

        it.next().unwrap();

        let instructions = it
            .next()
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect();

        return Computer {
            a,
            b,
            c,
            ip: 0,
            instructions,
            out: Vec::new(),
        };
    }

    fn get_combo_operand(&self) -> Register {
        let operand = *self.instructions.get(self.ip + 1).unwrap();
        match operand {
            0 | 1 | 2 | 3 => operand.into(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!(),
        }
    }

    fn get_literal_operand(&self) -> Register {
        (*self.instructions.get(self.ip + 1).unwrap()).into()
    }

    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.c = 0;
        self.ip = 0;
        self.out = Vec::new();
    }

    fn run(&mut self) -> bool {
        loop {
            let instruction = match self.instructions.get(self.ip) {
                None => break,
                Some(i) => *i,
            };

            match instruction {
                0 => {
                    self.a = self.a / (1 << self.get_combo_operand());
                }
                1 => {
                    self.b = self.b ^ self.get_literal_operand();
                }
                2 => {
                    self.b = self.get_combo_operand() % 8;
                }
                3 => {
                    if self.a != 0 {
                        self.ip = self.get_literal_operand().try_into().unwrap();
                        continue;
                    }
                }
                4 => {
                    self.b = self.b ^ self.c;
                }
                5 => {
                    self.out.push(self.get_combo_operand() % 8);
                    if self.out.len() > self.instructions.len() {
                        return false;
                    }

                    let l = self.out.len() - 1;
                    if self.out[l] != self.instructions[l].into() {
                        return false;
                    }
                }
                6 => {
                    self.b = self.a / (1 << self.get_combo_operand());
                }
                7 => {
                    self.c = self.a / (1 << self.get_combo_operand());
                }
                _ => panic!(),
            }

            self.ip += 2;
        }

        return self.out.len() == self.instructions.len()
            && self
                .out
                .iter()
                .zip(self.instructions.iter())
                .all(|(o, i)| *o == (*i).into());
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut computer = Computer::new(fs::read_to_string(&args[1]).unwrap());

    let mut a = 0;
    loop {
        /*
        if a % 100 == 0 {
            println!("{a}...");
        }
        */

        computer.reset();
        computer.a = a;

        if computer.run() {
            break;
        }

        a += 1;
    }

    println!("{a}");
}
