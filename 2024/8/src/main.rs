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

fn maybe_insert(city: &City, s: &mut HashSet<Pt>, p: Pt) {
    if p.0 >= 0 && p.1 >= 0 && p.0 <= city.max.0 && p.1 <= city.max.1 {
        s.insert(p);
    }
}

fn find_antinodes(city: &City) -> HashSet<Pt> {
    let mut r = HashSet::new();

    for (_c, v) in &city.antennas {
        for (a1, a2) in v.iter().cartesian_product(v.iter()) {
            if a1 == a2 {
                continue;
            }

            let d = (a2.0 - a1.0, a2.1 - a1.1);
            maybe_insert(city, &mut r, (a2.0 + d.0, a2.1 + d.1));
            maybe_insert(city, &mut r, (a1.0 - d.0, a1.1 - d.1));
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
