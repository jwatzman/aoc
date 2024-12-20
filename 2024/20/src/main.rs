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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Cheat {
    CanCheat,
    Cheating(usize),
    Cheated,
}
const CHEAT_STEPS: usize = 2;

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

fn adj<'a>(
    racetrack: &'a Racetrack,
    state: &'a (Pt, Cheat),
) -> impl Iterator<Item = (Pt, Cheat)> + use<'a> {
    Direction::ALL.iter().filter_map(move |d| {
        let pt = &state.0 + d.delta();
        let pos = aoc_util::try_get(&racetrack.map, &pt)?;
        match (*pos, state.1) {
            (Position::Track, Cheat::CanCheat) => Some((pt, Cheat::CanCheat)),
            (Position::Track, _) => Some((pt, Cheat::Cheated)),
            (Position::Wall, Cheat::CanCheat) => Some((pt, Cheat::Cheating(CHEAT_STEPS - 1))),
            (Position::Wall, Cheat::Cheating(n)) if n > 1 => Some((pt, Cheat::Cheating(n - 1))),
            (Position::Wall, _) => None,
        }
    })
}

fn solve(racetrack: &Racetrack, enable_cheats: bool) -> usize {
    let mut pq = priority_queue::PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut costs = HashMap::new();

    let approx_sz = racetrack.map.len() * racetrack.map.len();
    visited.reserve(approx_sz);
    costs.reserve(approx_sz);

    let init_state = (
        racetrack.start.clone(),
        if enable_cheats {
            Cheat::CanCheat
        } else {
            Cheat::Cheated
        },
    );

    costs.insert(init_state.clone(), 0);
    pq.push(init_state, Reverse(0));

    while let Some((state, _)) = pq.pop() {
        let cost = *costs.get(&state).unwrap();
        if state.0 == racetrack.end {
            return cost;
        }

        for next in adj(racetrack, &state) {
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

        visited.insert(state);
    }

    panic!();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let racetrack = parse_input(fs::read_to_string(&args[1]).unwrap());
    let baseline = solve(&racetrack, false);
    let fastest = solve(&racetrack, true);
    println!("{}", baseline - fastest);
}
