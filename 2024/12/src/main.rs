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

impl Pt {
    fn rot_left(&self) -> Pt {
        return Pt {
            row: -self.col,
            col: self.row,
        };
    }

    fn rot_right(&self) -> Pt {
        return Pt {
            row: self.col,
            col: -self.row,
        };
    }
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

fn find_area(start: &Pt, garden: &Garden, seen: &mut HashSet<Pt>) -> HashSet<Pt> {
    if seen.contains(start) {
        return HashSet::new();
    }
    seen.insert(start.clone());

    let flower = try_get(garden, start).unwrap();

    let mut area = HashSet::new();
    let mut todo = HashSet::new();
    todo.insert(start.clone());

    while todo.len() > 0 {
        let i = todo.into_iter();
        todo = HashSet::new();
        for pt in i {
            area.insert(pt.clone());
            for delta in &DELTAS {
                let new_pt = &pt + delta;
                match try_get(garden, &new_pt) {
                    Some(f) if f == flower => (),
                    _ => continue,
                };

                if !seen.contains(&new_pt) {
                    seen.insert(new_pt.clone());
                    todo.insert(new_pt);
                }
            }
        }
    }

    return area;
}

/*
fn right_hand_find_start(area: &HashSet<Pt>, d: &Pt) -> Option<Pt> {
    let mut cur = area.iter().next()?.clone();
    loop {
        let next = &cur + d;
        if !area.contains(&next) {
            return Some(cur);
        }
        cur = next;
    }
}
*/

const UP: Pt = Pt { row: -1, col: 0 };

fn right_hand(area: &HashSet<Pt>, start: &Pt, seen: &mut HashSet<Pt>) -> usize {
    let start_dir = UP.rot_left();

    let mut r = 0;
    let mut cur = start.clone();
    let mut cur_dir = start_dir.clone();

    loop {
        seen.insert(cur.clone());
        let dir_right = cur_dir.rot_right();
        let right = &cur + &dir_right;

        let forward = &cur + &cur_dir;

        if area.contains(&right) {
            r += 1;
            cur_dir = dir_right;
            cur = right;
        } else if area.contains(&forward) {
            cur = forward;
        } else {
            r += 1;
            cur_dir = cur_dir.rot_left();
        }

        if cur == *start && cur_dir == start_dir {
            break;
        }
    }

    return r;
}

fn right_hand_all(area: &HashSet<Pt>) -> usize {
    let mut r = 0;
    let mut seen = HashSet::new();
    for pt in area {
        let mut start = pt.clone();
        loop {
            let next = &start + &UP;
            if !area.contains(&next) {
                break;
            }
            start = next;
        }
        if !seen.contains(&start) {
            r += right_hand(area, &start, &mut seen);
        }
    }
    dbg!(&r);
    return r;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let garden = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut seen = HashSet::new();
    let mut r = 0;

    let max_row = garden.len();
    let max_col = garden[0].len();
    for row in 0..max_row {
        for col in 0..max_col {
            let pt = Pt {
                row: RC::try_from(row).unwrap(),
                col: RC::try_from(col).unwrap(),
            };

            let area = find_area(&pt, &garden, &mut seen);
            let perim = right_hand_all(&area);
            let cost = area.len() * perim;
            r += cost;
        }
    }

    println!("{r}");
}
