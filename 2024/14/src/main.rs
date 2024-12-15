use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;

type RC = i16;
type Pt = aoc_util::Pt<RC>;

const N_ROWS: RC = 103;
const N_COLS: RC = 101;
const EXTENT: Pt = Pt {
    row: N_ROWS,
    col: N_COLS,
};

#[derive(Debug)]
struct Robot {
    p: Pt,
    v: Pt,
}

impl Robot {
    fn step(&mut self, steps: RC) {
        self.p += &self.v * steps;
        self.p %= EXTENT;
    }
}

fn parse_input(contents: String) -> Vec<Robot> {
    let mut robots = Vec::new();
    let re = Regex::new(r"p=(.+),(.+) v=(.+),(.+)").unwrap();

    for line in contents.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let (_, [c1, c2, c3, c4]) = re.captures(line).unwrap().extract();

        robots.push(Robot {
            p: Pt {
                col: c1.parse().unwrap(),
                row: c2.parse().unwrap(),
            },
            v: Pt {
                col: c3.parse().unwrap(),
                row: c4.parse().unwrap(),
            },
        })
    }

    return robots;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut robots = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut tot_steps = 0;
    loop {
        let mut ps = HashMap::new();
        for robot in robots.iter_mut() {
            robot.step(1);
            ps.insert(robot.p.clone(), ps.get(&robot.p).unwrap_or(&0) + 1);
        }
        tot_steps += 1;

        if ps.iter().any(|(_, n)| *n > 1) {
            continue;
        }

        println!("{tot_steps}");
        for row in 0..N_ROWS {
            for col in 0..N_COLS {
                match ps.get(&Pt { row, col }) {
                    None => print!("."),
                    Some(n) => print!("{n}"),
                }
            }
            print!("\n");
        }

        print!("\n\n\n");
        break;
    }
}
