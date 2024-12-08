use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

type Pt = (i32, i32);

#[derive(Debug)]
struct City {
    antennas: HashMap<char, Vec<Pt>>,
    max: Pt,
}

fn parse_input(contents: String) -> City {
    let mut antennas = HashMap::new();
    let mut max = None;

    for (row, l) in contents.split('\n').enumerate() {
        for (col, c) in l.trim().chars().enumerate() {
            let row32 = i32::try_from(row).unwrap();
            let col32 = i32::try_from(col).unwrap();
            let p = (row32, col32);

            if c != '.' {
                match antennas.get_mut(&c) {
                    None => {
                        let mut v = Vec::new();
                        v.push(p);
                        antennas.insert(c, v);
                    }
                    Some(v) => v.push(p),
                }
            }

            max = Some(p);
        }
    }

    return City {
        antennas,
        max: max.unwrap(),
    };
}

// https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
fn gcd(mut n: i32, mut m: i32) -> i32 {
    if n == 0 {
        return m;
    }

    if m == 0 {
        return n;
    }

    if n < 0 {
        n = -n;
    }

    if m < 0 {
        m = -m;
    }

    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    return n;
}

fn scan_antinodes(city: &City, s: &mut HashSet<Pt>, mut r: i32, mut c: i32, dr: i32, dc: i32) {
    while r >= 0 && c >= 0 && r <= city.max.0 && c <= city.max.1 {
        s.insert((r, c));
        r += dr;
        c += dc;
    }
}

fn find_antinodes(city: &City) -> HashSet<Pt> {
    let mut r = HashSet::new();

    for (_c, v) in &city.antennas {
        for (a1, a2) in v.iter().cartesian_product(v.iter()) {
            if a1 == a2 {
                continue;
            }

            let dr_big = a2.0 - a1.0;
            let dc_big = a2.1 - a1.1;
            let g = gcd(dr_big, dc_big);
            let dr = dr_big / g;
            let dc = dc_big / g;

            scan_antinodes(city, &mut r, a1.0, a1.1, dr, dc);
            scan_antinodes(city, &mut r, a1.0, a1.1, -dr, -dc);
        }
    }

    return r;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let city = parse_input(fs::read_to_string(&args[1]).unwrap());
    let antinodes = find_antinodes(&city);
    // dbg!(&antinodes);
    let r = antinodes.len();
    println!("{r}");
}
