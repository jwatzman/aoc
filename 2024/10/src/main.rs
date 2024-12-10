use std::collections::HashSet;
use std::env;
use std::fs;
use std::ops::Add;

type RC = i8;
type Ht = i8;

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

const DELTAS: [Pt; 4] = [
    Pt { row: -1, col: 0 },
    Pt { row: 1, col: 0 },
    Pt { row: 0, col: -1 },
    Pt { row: 0, col: 1 },
];

fn parse_input(contents: String) -> (Vec<Vec<Ht>>, HashSet<Pt>) {
    let mut topo = Vec::new();
    let mut heads = HashSet::new();

    for (row, l) in contents.split('\n').enumerate() {
        if l.len() == 0 {
            continue;
        }

        let mut row_vec = Vec::new();
        for (col, c) in l.trim().chars().enumerate() {
            let h: Ht = c.to_digit(10).unwrap().try_into().unwrap();
            row_vec.push(h);

            let p = Pt {
                row: row.try_into().unwrap(),
                col: col.try_into().unwrap(),
            };
            if h == 0 {
                heads.insert(p);
            }
        }

        topo.push(row_vec);
    }

    return (topo, heads);
}

fn try_get(topo: &Vec<Vec<i8>>, pt: &Pt) -> Option<Ht> {
    let row = Result::ok(usize::try_from(pt.row))?;
    let col = Result::ok(usize::try_from(pt.col))?;
    return Some(*topo.get(row)?.get(col)?);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (topo, heads) = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut score = 0;
    for head in heads {
        let mut todo = HashSet::from([head]);
        for level in 1..10 {
            let mut new_todo = HashSet::new();
            for pt in &todo {
                for d in &DELTAS {
                    let new_pt = pt + d;
                    let h = match try_get(&topo, &new_pt) {
                        Some(h) => h,
                        None => continue,
                    };
                    if h == level {
                        new_todo.insert(new_pt);
                    }
                }
            }
            todo = new_todo;
        }

        score += todo.len();
    }

    println!("{score}");
}
