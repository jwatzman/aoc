use regex::Regex;
use std::env;
use std::fs;

#[derive(Debug)]
struct Axes {
    x: u32,
    y: u32,
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
            x: captures_prize[1].parse().unwrap(),
            y: captures_prize[2].parse().unwrap(),
        };

        r.push(Machine { a, b, prize });
    }

    return r;
}

const EPS: f64 = 0.00001;
fn solve(machine: &Machine) -> Option<(u32, u32)> {
    let m = nalgebra::Matrix2::new(
        f64::from(machine.a.x),
        f64::from(machine.b.x),
        f64::from(machine.a.y),
        f64::from(machine.b.y),
    );
    let v = nalgebra::Vector2::new(f64::from(machine.prize.x), f64::from(machine.prize.y));
    let decomp = m.lu();
    let s = decomp.solve(&v).unwrap();

    let a_f = s[0];
    let b_f = s[1];

    if a_f > -EPS
        && a_f < 100.
        && b_f > -EPS
        && b_f < 100.
        && (a_f.fract() < EPS || 1. - a_f.fract() < EPS)
        && (b_f.fract() < EPS || 1. - b_f.fract() < EPS)
    {
        return Some((a_f.round() as u32, b_f as u32));
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
