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

    fn run(&mut self) {
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
    }
}

fn part2(a: u64, program: &[u64]) -> Option<u64> {
    let out = match program.first() {
        None => return Some(a),
        Some(out) => *out,
    };

    for i in 0..8 {
        let maybe_a = (a << 3) + i;
        if out == (maybe_a ^ 5 ^ (maybe_a >> (i ^ 1))) % 8 {
            match part2(maybe_a, &program[1..]) {
                None => (),
                Some(a) => return Some(a),
            }
        }
    }

    None
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() > 1 {
        let mut computer = Computer::new(fs::read_to_string(&args[1]).unwrap());
        computer.run();
        println!(
            "{}",
            computer
                .out
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",")
        );
    } else {
        let mut program = vec![2_u64, 4, 1, 1, 7, 5, 4, 6, 0, 3, 1, 4, 5, 5, 3, 0];
        program.reverse();
        let r = part2(0, &program);
        dbg!(&r);
    }
}
