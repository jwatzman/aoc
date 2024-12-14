use regex::Regex;
use std::env;
use std::fs;

type NN = u64;

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

// https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
fn gcd(mut n: u64, mut m: u64) -> u64 {
    if n == 0 {
        return m;
    }

    if m == 0 {
        return n;
    }

    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    return n;
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
            x: captures_prize[1].parse::<NN>().unwrap() + 10000000000000,
            y: captures_prize[2].parse::<NN>().unwrap() + 10000000000000,
        };

        r.push(Machine { a, b, prize });
    }

    return r;
}

fn fconv(n: NN) -> f64 {
    //f64::from(u32::try_from(n).unwrap())
    n as f64
}

const EPS: f64 = 0.00001;
fn solve(machine: &Machine) -> Option<(NN, NN)> {
    let gcd_x = gcd(gcd(machine.a.x, machine.b.x), machine.prize.x);
    let gcd_y = gcd(gcd(machine.a.y, machine.b.y), machine.prize.y);
    dbg!(&gcd_x, &gcd_y);

    let m = nalgebra::Matrix2::new(
        fconv(machine.a.x / gcd_x),
        fconv(machine.b.x / gcd_x),
        fconv(machine.a.y / gcd_y),
        fconv(machine.b.y / gcd_y),
    );
    let v = nalgebra::Vector2::new(
        fconv(machine.prize.x / gcd_x),
        fconv(machine.prize.y / gcd_y),
    );
    let decomp = m.lu();
    let s = decomp.solve(&v).unwrap();

    let a_f = s[0];
    let b_f = s[1];

    if a_f > -EPS
        && b_f > -EPS
        && (a_f.fract() < EPS || 1. - a_f.fract() < EPS)
        && (b_f.fract() < EPS || 1. - b_f.fract() < EPS)
    {
        return Some((a_f.round() as NN, b_f.round() as NN));
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
