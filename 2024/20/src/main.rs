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

fn count_cheats(racetrack: &Racetrack, cheat_time: RC, cheat_improvement: RC) -> usize {
    let path = flatten(racetrack);
    let mut cnt = 0;

    for i in 0..path.len() {
        let start = &path[i];
        for j in (i + 1)..path.len() {
            let end = &path[j];

            let cheated_distance = (end - start).manhattan_len();
            if cheated_distance > cheat_time {
                continue;
            }

            let improvement = i16::try_from(j - i).unwrap() - cheated_distance;
            if improvement >= cheat_improvement {
                cnt += 1;
            }
        }
    }

    return cnt;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let racetrack = parse_input(fs::read_to_string(&args[1]).unwrap());

    let cheats = count_cheats(&racetrack, 20, 100);
    println!("{cheats}");
}
