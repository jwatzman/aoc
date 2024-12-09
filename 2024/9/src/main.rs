use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Block {
    Free,
    File(u32),
}

#[allow(dead_code)]
fn p(v: &Vec<Block>) {
    for b in v {
        match b {
            Block::Free => print!("."),
            Block::File(n) => print!("{n}"),
        }
    }
    print!("\n");
}

fn parse_input(contents: String) -> Vec<Block> {
    let mut r = Vec::new();

    let mut free = false;
    let mut next_file_id = 0;

    for c in contents.chars() {
        let n = match c.to_digit(10) {
            None => break,
            Some(n) => n,
        };

        let b = if free {
            Block::Free
        } else {
            next_file_id += 1;
            Block::File(next_file_id - 1)
        };
        free = !free;

        for _i in 0..n {
            r.push(b.clone());
        }
    }

    return r;
}

fn compact(blocks: &Vec<Block>) -> Vec<Block> {
    let mut res = blocks.clone();

    let mut l = 0_usize;
    let mut r = res.len() - 1;

    while l < r {
        match (res[l].clone(), res[r].clone()) {
            (Block::File(_), _) => l += 1,
            (Block::Free, Block::Free) => r -= 1,
            (Block::Free, Block::File(_)) => {
                res.swap(l, r);
                l += 1;
                r -= 1;
            }
        }
    }

    return res;
}

fn cksum(blocks: &Vec<Block>) -> usize {
    let mut r = 0;
    for (pos, b) in blocks.iter().enumerate() {
        match b {
            Block::Free => (),
            Block::File(id) => r += pos * usize::try_from(*id).unwrap(),
        }
    }

    return r;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let blocks = parse_input(fs::read_to_string(&args[1]).unwrap());
    // p(&blocks);
    let compacted = compact(&blocks);
    // p(&compacted);
    let ck = cksum(&compacted);
    println!("{ck}");
}
