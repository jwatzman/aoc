use std::env;
use std::fs;

const DELTAS: [i8; 3] = [-1, 0, 1];

fn find(grid: &Vec<Vec<char>>, cs: &[char], delta: &(i8, i8), loc: &(usize, usize)) -> Option<()> {
    if cs.len() == 0 {
        return Some(());
    }

    let new_loc = (
        loc.0.checked_add_signed(delta.0.into())?,
        loc.1.checked_add_signed(delta.1.into())?,
    );
    let c = grid.get(new_loc.0)?.get(new_loc.1)?;
    if *c == cs[0] {
        return find(grid, &cs[1..], delta, &new_loc);
    } else {
        return None;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).unwrap();

    let mut grid = Vec::new();
    for line in contents.split('\n') {
        let mut cs = Vec::new();
        for c in line.chars() {
            cs.push(c);
        }
        grid.push(cs);
    }

    let mut res = 0;
    for (rn, r) in grid.iter().enumerate() {
        for (cn, c) in r.iter().enumerate() {
            let loc = (rn, cn);
            for rd in DELTAS {
                for cd in DELTAS {
                    if !(rd == 0 && cd == 0)
                        && *c == 'X'
                        && find(&grid, &['M', 'A', 'S'], &(rd, cd), &loc).is_some()
                    {
                        res += 1;
                        // println!("{rn}, {cn} + {rd}, {cd}");
                    }
                }
            }
        }
    }

    println!("{res}");
}
