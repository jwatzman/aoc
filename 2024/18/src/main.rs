use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use aoc_util::Direction;

type RC = i16;
type Pt = aoc_util::Pt<RC>;

const MAX_ROW: RC = 70;
const MAX_COL: RC = 70;

fn parse_input(contents: String) -> Vec<Pt> {
    let mut corrupted = Vec::new();

    for line in contents.split('\n') {
        if line.len() == 0 {
            continue;
        }

        let mut it = line.split(',');
        corrupted.push(Pt {
            row: it.next().unwrap().parse().unwrap(),
            col: it.next().unwrap().parse().unwrap(),
        })
    }

    return corrupted;
}

fn adj<'a>(corrupted: &'a HashSet<Pt>, pt: &'a Pt) -> impl Iterator<Item = Pt> + use<'a> {
    Direction::ALL
        .iter()
        .map(move |d| pt + d.delta())
        .filter(|p| p.row >= 0 && p.col >= 0)
        .filter(|p| p.row <= MAX_ROW && p.col <= MAX_COL)
        .filter(|p| !corrupted.contains(p))
}

fn solve(corrupted: &HashSet<Pt>) -> Option<usize> {
    let mut pq = priority_queue::PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut costs = HashMap::new();

    let start = Pt { row: 0, col: 0 };
    costs.insert(start.clone(), 0);
    pq.push(start, Reverse(0));

    while let Some((pt, _)) = pq.pop() {
        let cost = *costs.get(&pt).unwrap();
        if pt.row == MAX_ROW && pt.col == MAX_COL {
            return Some(cost);
        }

        for next in adj(corrupted, &pt) {
            let tot_cost = cost + 1;
            let prev_tot_cost = *costs.get(&next).unwrap_or(&usize::MAX);
            if tot_cost < prev_tot_cost {
                costs.insert(next.clone(), tot_cost);
                pq.push(next, Reverse(tot_cost));
            }
        }

        visited.insert(pt);
    }

    return None;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let corrupted = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut cur: HashSet<Pt> = corrupted[0..1024].iter().cloned().collect();
    for corruption in &corrupted[1024..] {
        cur.insert(corruption.clone());
        match solve(&cur) {
            Some(_) => continue,
            None => {
                dbg!(corruption);
                break;
            }
        }
    }
}
