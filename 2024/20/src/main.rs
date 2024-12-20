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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Cheat {
    CanCheat(usize),
    Cheating(usize, Pt),
    Cheated(Pt, Pt),
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

fn adj<'a>(
    racetrack: &'a Racetrack,
    state: &'a (Pt, Cheat),
) -> impl Iterator<Item = (Pt, Cheat)> + use<'a> {
    Direction::ALL.iter().filter_map(move |d| {
        let pt = &state.0 + d.delta();
        let pos = aoc_util::try_get(&racetrack.map, &pt)?;
        match (*pos, state.1.clone()) {
            (Position::Track, Cheat::CanCheat(_)) => Some((pt, state.1.clone())),
            (Position::Track, Cheat::Cheating(_, start)) => {
                Some((pt.clone(), Cheat::Cheated(start, pt.clone())))
            }
            (Position::Track, Cheat::Cheated(_, _)) => Some((pt, state.1.clone())),
            (Position::Wall, Cheat::CanCheat(n)) => {
                Some((pt.clone(), Cheat::Cheating(n - 1, pt.clone())))
            }
            (Position::Wall, Cheat::Cheating(n, start)) if n > 1 => {
                Some((pt, Cheat::Cheating(n - 1, start)))
            }
            (Position::Wall, Cheat::Cheating(_, _)) => None,
            (Position::Wall, Cheat::Cheated(_, _)) => None,
        }
    })
}

fn solve(racetrack: &Racetrack, init_cheat: Cheat) -> (usize, (Pt, Pt)) {
    let mut pq = priority_queue::PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut costs = HashMap::new();
    let mut prevs = HashMap::new();

    let approx_sz = racetrack.map.len() * racetrack.map.len();
    visited.reserve(approx_sz);
    costs.reserve(approx_sz);

    let init_state = (racetrack.start.clone(), init_cheat);

    costs.insert(init_state.clone(), 0);
    pq.push(init_state, Reverse(0));

    while let Some((state, _)) = pq.pop() {
        let cost = *costs.get(&state).unwrap();
        if state.0 == racetrack.end {
            return (
                cost,
                match state.1 {
                    Cheat::Cheated(start, end) => (start, end),
                    _ => panic!(),
                },
            );
        }

        for next in adj(racetrack, &state) {
            let tot_cost = cost + 1;

            let prev_tot_cost_opt = costs.get_mut(&next);
            match prev_tot_cost_opt {
                None => {
                    costs.insert(next.clone(), tot_cost);
                    prevs.insert(next.clone(), state.clone());
                    pq.push(next, Reverse(tot_cost));
                }
                Some(prev_tot_cost) => {
                    if tot_cost < *prev_tot_cost {
                        *prev_tot_cost = tot_cost;
                        prevs.insert(next.clone(), state.clone());
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

    let dummy_pt = Pt { row: 0, col: 0 };
    let (baseline, _) = solve(
        &racetrack,
        Cheat::Cheated(dummy_pt.clone(), dummy_pt.clone()),
    );

    let (fastest, cheat_used) = solve(&racetrack, Cheat::CanCheat(2));
    println!("{}", baseline - fastest);
    dbg!(cheat_used);
}
