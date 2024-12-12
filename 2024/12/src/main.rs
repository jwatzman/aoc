use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::Add;

type RC = i16;
type Garden = Vec<Vec<char>>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pt {
    row: RC,
    col: RC,
}

impl Add for Pt {
    type Output = Pt;
    fn add(self, other: Pt) -> Pt {
        Pt {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

impl Add for &Pt {
    type Output = Pt;
    fn add(self, other: &Pt) -> Pt {
        Pt {
            row: self.row + other.row,
            col: self.col + other.col,
        }
    }
}

fn try_get<T: Copy>(vv: &Vec<Vec<T>>, pt: &Pt) -> Option<T> {
    let row = Result::ok(usize::try_from(pt.row))?;
    let col = Result::ok(usize::try_from(pt.col))?;
    return Some(*vv.get(row)?.get(col)?);
}

const DELTAS: [Pt; 4] = [
    Pt { row: -1, col: 0 },
    Pt { row: 1, col: 0 },
    Pt { row: 0, col: -1 },
    Pt { row: 0, col: 1 },
];

fn parse_input(contents: String) -> Garden {
    let mut garden = Vec::new();
    for l in contents.split('\n') {
        if l.len() == 0 {
            continue;
        }

        let mut row_vec = Vec::new();
        for c in l.trim().chars() {
            row_vec.push(c);
        }
        garden.push(row_vec);
    }

    return garden;
}

fn scan(start: &Pt, garden: &Garden, seen: &mut HashSet<Pt>) -> (u16, u16) {
    if seen.contains(start) {
        return (0, 0);
    }
    seen.insert(start.clone());

    let flower = try_get(garden, start).unwrap();
    let mut area = 0;
    let mut perim = 0;

    let mut todo = HashSet::new();
    todo.insert(start.clone());

    while todo.len() > 0 {
        let i = todo.into_iter();
        todo = HashSet::new();
        for pt in i {
            area += 1;
            perim += 4;

            for delta in &DELTAS {
                let new_pt = &pt + delta;
                match try_get(garden, &new_pt) {
                    Some(f) if f == flower => (),
                    _ => continue,
                };

                perim -= 1;

                if !seen.contains(&new_pt) {
                    seen.insert(new_pt.clone());
                    todo.insert(new_pt);
                }
            }
        }
    }

    return (area, perim);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let garden = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut seen = HashSet::new();
    let mut r = 0_u64;

    let max_row = garden.len();
    let max_col = garden[0].len();
    for row in 0..max_row {
        for col in 0..max_col {
            let pt = Pt {
                row: RC::try_from(row).unwrap(),
                col: RC::try_from(col).unwrap(),
            };

            let (area, perim) = scan(&pt, &garden, &mut seen);
            let cost: u64 = (area * perim).into();
            r += cost;
        }
    }

    println!("{r}");
}
