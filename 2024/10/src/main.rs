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

fn parse_input(contents: String) -> (Vec<Vec<Ht>>, HashSet<Pt>, Pt) {
    let mut topo = Vec::new();
    let mut heads = HashSet::new();
    let mut max = Pt { row: 0, col: 0 };

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
                heads.insert(p.clone());
            }
            max = p;
        }

        topo.push(row_vec);
    }

    return (topo, heads, max);
}

/*
fn maybe_get(topo: &Vec<Vec<i8>>, pt: &Pt) -> Option<Ht> {
    if pt.row < 0 || pt.col < 0 {
        return None;
    }

    let row: Option<&[Ht]> = topo.get(pt.row.into());
    let ht: &Ht = row?.get(pt.col.into())?;
    return Some(*ht);
}
*/

fn main() {
    let args: Vec<_> = env::args().collect();
    let (topo, heads, max) = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut score = 0;
    for head in heads {
        let mut todo = HashSet::from([head]);
        for level in 1..10 {
            let mut new_todo = HashSet::new();
            for pt in &todo {
                for d in &DELTAS {
                    let new_pt = pt + d;
                    if new_pt.row < 0
                        || new_pt.col < 0
                        || new_pt.row > max.row
                        || new_pt.col > max.col
                    {
                        continue;
                    }

                    let h = topo[usize::try_from(new_pt.row).unwrap()]
                        [usize::try_from(new_pt.col).unwrap()];
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
