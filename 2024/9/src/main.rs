use std::env;
use std::fs;

#[derive(Debug, Clone)]
enum Block {
    Free(u32),
    File(u32, u32),
}

#[allow(dead_code)]
fn p(v: &Vec<Block>) {
    for b in v {
        let (len, c) = match b {
            Block::Free(len) => (*len, '.'),
            Block::File(len, id) => (*len, id.to_string().chars().nth(0).unwrap()),
        };

        for _ in 0..len {
            print!("{c}");
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
            Block::Free(n)
        } else {
            next_file_id += 1;
            Block::File(n, next_file_id - 1)
        };
        free = !free;

        r.push(b.clone());
    }

    return r;
}

fn compact(blocks: &Vec<Block>) -> Vec<Block> {
    let mut blocks = blocks.clone();

    let mut next_file_id = match blocks.last() {
        Some(Block::File(_len, id)) => *id,
        _ => panic!("Invalid blocks"),
    };

    while next_file_id > 0 {
        let mut pos = blocks.len() - 1;
        let len;
        loop {
            //dbg!(&blocks[pos]);
            match blocks[pos] {
                Block::File(l, id) if id == next_file_id => {
                    len = l;
                    break;
                }
                _ => pos -= 1,
            }
        }

        let mut target = 0;
        while target < pos {
            match blocks[target] {
                Block::Free(flen) if flen >= len => {
                    blocks[pos] = Block::Free(len);
                    blocks[target] = Block::File(len, next_file_id);
                    if flen > len {
                        blocks.insert(target + 1, Block::Free(flen - len));
                    }
                    break;
                }
                _ => target += 1,
            }
        }

        next_file_id -= 1;
    }

    return blocks;
}

fn cksum(blocks: &Vec<Block>) -> u64 {
    let mut r = 0_u64;
    let mut pos = 0;
    for b in blocks.iter() {
        match b {
            Block::Free(len) => pos += len,
            Block::File(len, id) => {
                for _ in 0..*len {
                    match r.checked_add((pos * id).into()) {
                        Some(n) => r = n,
                        None => panic!("{} {} {}", r, pos, id),
                    }
                    pos += 1;
                }
            }
        }
    }

    return r;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let blocks = parse_input(fs::read_to_string(&args[1]).unwrap());
    //p(&blocks);
    let compacted = compact(&blocks);
    //p(&compacted);
    let ck = cksum(&compacted);
    println!("{ck}");
}
