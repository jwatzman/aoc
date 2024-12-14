use regex::Regex;
use std::env;
use std::fs;

type NN = i64;

#[derive(Debug)]
struct Axes {
    x: NN,
    y: NN,
}

#[derive(Debug)]
struct Machine {
    a: Axes,
    b: Axes,
    prize: Axes,
}

fn parse_input(contents: String) -> Vec<Machine> {
    let mut r = Vec::new();

    let re_ab = Regex::new(r"Button (A|B): X\+(.+), Y\+(.+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(.+), Y=(.+)").unwrap();

    let mut it = contents.split('\n');
    loop {
        let line_a = match it.next() {
            Some(l) => l,
            None => break,
        };

        let line_b = it.next().unwrap();
        let line_prize = it.next().unwrap();
        it.next();

        let captures_a = re_ab.captures(line_a).unwrap();
        let a = Axes {
            x: captures_a[2].parse().unwrap(),
            y: captures_a[3].parse().unwrap(),
        };

        let captures_b = re_ab.captures(line_b).unwrap();
        let b = Axes {
            x: captures_b[2].parse().unwrap(),
            y: captures_b[3].parse().unwrap(),
        };

        let captures_prize = re_prize.captures(line_prize).unwrap();
        let prize = Axes {
            x: captures_prize[1].parse::<NN>().unwrap(), /*+ 10000000000000*/
            y: captures_prize[2].parse::<NN>().unwrap(), /*+ 10000000000000*/
        };

        r.push(Machine { a, b, prize });
    }

    return r;
}

fn solve(machine: &Machine) -> Option<(NN, NN)> {
    let det = machine.a.x * machine.b.y - machine.b.x * machine.a.y;
    let a = (machine.prize.x * machine.b.y - machine.b.x * machine.prize.y) / det;
    let b = (machine.a.x * machine.prize.y - machine.prize.x * machine.a.y) / det;

    if machine.a.x * a + machine.b.x * b == machine.prize.x
        && machine.a.y * a + machine.b.y * b == machine.prize.y
    {
        return Some((a, b));
    } else {
        return None;
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let machines = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut r = 0;
    for machine in &machines {
        match solve(machine) {
            None => (),
            Some((a, b)) => r += 3 * a + b,
        }
    }

    println!("{r}");
}
