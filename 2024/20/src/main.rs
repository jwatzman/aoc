use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

use aoc_util::try_get;
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

fn flatten(racetrack: &Racetrack) -> Vec<Pt> {
    let mut cur = racetrack.start.clone();
    let mut prev = Pt { row: -1, col: -1 };
    let mut path = Vec::new();

    path.push(cur.clone());

    while cur != racetrack.end {
        for delta in Direction::ALL.map(|d| d.delta()) {
            let next = &cur + delta;
            if next == prev {
                continue;
            }

            let pos = try_get(&racetrack.map, &next);
            if *pos.unwrap_or(&Position::Wall) == Position::Wall {
                continue;
            }

            prev = cur;
            cur = next;
            path.push(cur.clone());
        }
    }

    return path;
}

fn invert(path: &Vec<Pt>) -> HashMap<Pt, isize> {
    path.iter()
        .cloned()
        .enumerate()
        .map(|(a, b)| (b, a.try_into().unwrap()))
        .collect()
}

fn find_cheat_destinations(
    racetrack: &Racetrack,
    start: &Pt,
    cheat_time: isize,
) -> HashSet<(Pt, isize)> {
    let init_cheat_time = cheat_time;
    let mut cheat_time = init_cheat_time;
    let mut todo = vec![start.clone()];
    let mut todo_next;
    let mut visited = HashSet::new();
    let mut dests = HashSet::new();

    while !todo.is_empty() && cheat_time > 0 {
        todo_next = Vec::new();
        cheat_time -= 1;

        for cur in todo {
            visited.insert(cur.clone());

            for next in Direction::ALL.map(|d| &cur + d.delta()) {
                if visited.contains(&next) {
                    continue;
                }

                match try_get(&racetrack.map, &next) {
                    None => continue,
                    Some(Position::Wall) => {
                        todo_next.push(next);
                    }
                    Some(Position::Track) => {
                        dests.insert((next, init_cheat_time - cheat_time));
                    }
                }
            }
        }

        todo = todo_next;
    }

    return dests;
}

fn count_cheats(racetrack: &Racetrack, cheat_time: isize, cheat_improvement: isize) -> usize {
    let path = flatten(racetrack);
    let inverted_path = invert(&path);
    let mut cnt = 0;

    for cheat_start in path {
        for (cheat_dest, cheated_steps) in
            find_cheat_destinations(racetrack, &cheat_start, cheat_time)
        {
            let uncheated_steps =
                inverted_path.get(&cheat_dest).unwrap() - inverted_path.get(&cheat_start).unwrap();
            //dbg!(&cheat_start, &cheat_dest, &uncheated_steps, &cheated_steps);
            match uncheated_steps.checked_sub(cheated_steps) {
                None => (),
                Some(n) => {
                    if n >= cheat_improvement {
                        cnt += 1;
                    }
                }
            }
        }
    }

    return cnt;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let racetrack = parse_input(fs::read_to_string(&args[1]).unwrap());

    let cheats = count_cheats(&racetrack, 2, 100);
    println!("{cheats}");
}
