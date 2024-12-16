use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;

type RC = i16;
type Pt = aoc_util::Pt<RC>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rot_left(&self) -> Self {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn rot_right(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn delta(&self) -> Pt {
        match *self {
            Direction::Up => Pt { row: -1, col: 0 },
            Direction::Down => Pt { row: 1, col: 0 },
            Direction::Left => Pt { row: 0, col: -1 },
            Direction::Right => Pt { row: 0, col: 1 },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
}

type Maze = Vec<Vec<Tile>>;

fn parse_input(contents: String) -> (Maze, Pt, Pt) {
    let mut maze = Vec::new();
    let mut start = None;
    let mut end = None;

    for (row, line) in contents.split('\n').into_iter().enumerate() {
        let mut maze_row = Vec::new();

        for (col, c) in line.chars().enumerate() {
            let tile = match c {
                '#' => Tile::Wall,
                '.' => Tile::Open,
                'S' => {
                    start = Some(Pt::try_from(row, col).unwrap());
                    Tile::Open
                }
                'E' => {
                    end = Some(Pt::try_from(row, col).unwrap());
                    Tile::Open
                }
                _ => panic!(),
            };
            maze_row.push(tile);
        }

        maze.push(maze_row);
    }

    return (maze, start.unwrap(), end.unwrap());
}

fn adj(maze: &Maze, (pt, dir): &(Pt, Direction)) -> Vec<((Pt, Direction), usize)> {
    let mut r = Vec::new();

    let forward = pt + dir.delta();
    match aoc_util::try_get(maze, &forward) {
        Some(Tile::Open) => r.push(((forward, *dir), 1)),
        _ => (),
    }

    r.push(((pt.clone(), dir.rot_left()), 1000));
    r.push(((pt.clone(), dir.rot_right()), 1000));

    return r;
}

fn solve(maze: &Maze, start: &Pt, end: &Pt) -> usize {
    let mut pq = priority_queue::PriorityQueue::new();
    let mut visited = HashSet::new();
    let mut costs = HashMap::new();

    let start_loc = (start.clone(), Direction::Right);
    costs.insert(start_loc.clone(), 0);
    pq.push(start_loc, Reverse(0));

    while let Some((loc, _)) = pq.pop() {
        let cost = *costs.get(&loc).unwrap();
        if loc.0 == *end {
            return cost;
        }

        for (next_loc, addl_cost) in adj(maze, &loc) {
            if visited.contains(&next_loc) {
                continue;
            }

            let tot_cost = cost + addl_cost;
            let prev_tot_cost = *costs.get(&next_loc).unwrap_or(&usize::MAX);

            if tot_cost < prev_tot_cost {
                costs.insert(next_loc.clone(), tot_cost);
                pq.push(next_loc, Reverse(tot_cost));
            }
        }

        visited.insert(loc);
    }

    panic!();
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (maze, start, end) = parse_input(fs::read_to_string(&args[1]).unwrap());
    let soln = solve(&maze, &start, &end);
    println!("{soln}");
}
