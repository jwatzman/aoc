use std::env;
use std::fs;

fn find(grid: &Vec<Vec<char>>, loc: &(usize, usize)) -> Option<()> {
    let up = loc.0.checked_add_signed(-1)?;
    let dn = loc.0.checked_add_signed(1)?;
    let lt = loc.1.checked_add_signed(-1)?;
    let rt = loc.1.checked_add_signed(1)?;

    let ul = *grid.get(up)?.get(lt)?;
    let ur = *grid.get(up)?.get(rt)?;
    let dl = *grid.get(dn)?.get(lt)?;
    let dr = *grid.get(dn)?.get(rt)?;

    if (ul == ur) && (dl == dr) {
        if (ul == 'M' && dl == 'S') || (ul == 'S' && dl == 'M') {
            return Some(());
        }
    }

    if (ul == dl) && (ur == dr) {
        if (ul == 'M' && ur == 'S') || (ul == 'S' && ur == 'M') {
            return Some(());
        }
    }

    None
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
            if *c == 'A' && find(&grid, &loc).is_some() {
                res += 1;
            }
        }
    }

    println!("{res}");
}
