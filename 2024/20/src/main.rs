use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use aoc_util::Direction;

type RC = i16;
type Pt = aoc_util::Pt<RC>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Position {
    Track,
    Wall,
}

#[derive(Debug)]
struct Racetrack {
    map: Vec<Vec<Position>>,
    start: Pt,
    end: Pt,
}

fn parse_input(contents: String) -> Racetrack {
    let mut map = Vec::new();
    let mut start = None;
    let mut end = None;

    for (row, line) in contents.split('\n').enumerate() {
        let mut map_row = Vec::new();

        for (col, c) in line.chars().enumerate() {
            map_row.push(match c {
                '.' => Position::Track,
                '#' => Position::Wall,
                'S' => {
                    start = Some(Pt::try_from(row, col).unwrap());
                    Position::Track
                }
                'E' => {
                    end = Some(Pt::try_from(row, col).unwrap());
                    Position::Track
                }
                _ => panic!(),
            })
        }

        map.push(map_row);
    }

    return Racetrack {
        map,
        start: start.unwrap(),
        end: end.unwrap(),
    };
}

fn adj<'a>(racetrack: &'a Racetrack, pt: &'a Pt) -> impl Iterator<Item = Pt> + use<'a> {
    Direction::ALL.iter().filter_map(move |d| {
        let pt = pt + d.delta();
        let pos = aoc_util::try_get(&racetrack.map, &pt)?;
        if *pos == Position::Track {
            Some(pt)
        } else {
            None
        }
    })
}

fn solve(racetrack: &Racetrack) -> usize {
    let mut pq = priority_queue::PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut costs = HashMap::new();

    let approx_sz = racetrack.map.len() * racetrack.map.len();
    visited.reserve(approx_sz);
    costs.reserve(approx_sz);

    costs.insert(racetrack.start.clone(), 0);
    pq.push(racetrack.start.clone(), Reverse(0));

    while let Some((pt, _)) = pq.pop() {
        let cost = *costs.get(&pt).unwrap();
        if pt == racetrack.end {
            return cost;
        }

        for next in adj(racetrack, &pt) {
            let tot_cost = cost + 1;

            let prev_tot_cost_opt = costs.get_mut(&next);
            match prev_tot_cost_opt {
                None => {
                    costs.insert(next.clone(), tot_cost);
                    pq.push(next, Reverse(tot_cost));
                }
                Some(prev_tot_cost) => {
                    if tot_cost < *prev_tot_cost {
                        *prev_tot_cost = tot_cost;
                        pq.push(next, Reverse(tot_cost));
                    }
                }
            }
        }

        visited.insert(pt);
    }

    panic!();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut racetrack = parse_input(fs::read_to_string(&args[1]).unwrap());
    let baseline = solve(&racetrack);

    let mut r = 0;
    for row in 0..racetrack.map.len() {
        for col in 0..racetrack.map[row].len() {
            let pos = racetrack.map.get_mut(row).unwrap().get_mut(col).unwrap();
            if *pos != Position::Wall {
                continue;
            }

            *pos = Position::Track;

            let cheated = solve(&racetrack);
            if baseline - cheated >= 100 {
                r += 1;
            }

            racetrack.map[row][col] = Position::Wall;
        }
    }

    println!("{r}");
}
