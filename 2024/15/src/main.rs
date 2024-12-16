use std::env;
use std::fmt::Display;
use std::fs;

use aoc_util::try_get;

type RC = i16;
type Pt = aoc_util::Pt<RC>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Item {
    Wall,
    BoxL,
    BoxR,
    Empty,
}

#[derive(Debug, Clone, Copy)]
enum Command {
    Up,
    Down,
    Left,
    Right,
}

impl Command {
    fn delta(&self) -> Pt {
        match *self {
            Command::Up => Pt { row: -1, col: 0 },
            Command::Down => Pt { row: 1, col: 0 },
            Command::Left => Pt { row: 0, col: -1 },
            Command::Right => Pt { row: 0, col: 1 },
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    items: Vec<Vec<Item>>,
    robot: Pt,
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (row, line) in self.items.iter().enumerate() {
            for (col, item) in line.iter().enumerate() {
                if row == self.robot.row.try_into().unwrap()
                    && col == self.robot.col.try_into().unwrap()
                {
                    write!(f, "@")?;
                } else {
                    let c = match item {
                        Item::Wall => '#',
                        Item::BoxL => '[',
                        Item::BoxR => ']',
                        Item::Empty => '.',
                    };
                    write!(f, "{c}")?;
                }
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn parse_input(contents: String) -> (Warehouse, Vec<Command>) {
    let mut items = Vec::new();
    let mut maybe_robot = None;

    let mut it = contents.split('\n').into_iter().enumerate();
    loop {
        let (row, line) = it.next().unwrap();
        if line.len() == 0 {
            break;
        }

        let mut line_items = Vec::new();
        for (col, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    line_items.push(Item::Empty);
                    line_items.push(Item::Empty);
                }
                '#' => {
                    line_items.push(Item::Wall);
                    line_items.push(Item::Wall);
                }
                'O' => {
                    line_items.push(Item::BoxL);
                    line_items.push(Item::BoxR);
                }
                '@' => {
                    line_items.push(Item::Empty);
                    line_items.push(Item::Empty);
                    maybe_robot = Some(Pt {
                        row: RC::try_from(row).unwrap(),
                        col: RC::try_from(col * 2).unwrap(),
                    });
                }
                _ => panic!("Invalid item: {}", c),
            }
        }

        items.push(line_items);
    }

    let mut commands = Vec::new();
    for (_, line) in it {
        for c in line.chars() {
            commands.push(match c {
                '^' => Command::Up,
                'v' => Command::Down,
                '<' => Command::Left,
                '>' => Command::Right,
                _ => panic!("Invalid command: {}", c),
            });
        }
    }

    return (
        Warehouse {
            items,
            robot: maybe_robot.unwrap(),
        },
        commands,
    );
}

fn exec_command(warehouse: &mut Warehouse, command: Command) {
    match command {
        Command::Left | Command::Right => {
            let delta = command.delta();
            let mut push_dest = &warehouse.robot + &delta;
            loop {
                match aoc_util::try_get(&warehouse.items, &push_dest).unwrap() {
                    Item::Wall => return,
                    Item::Empty => break,
                    Item::BoxL | Item::BoxR => push_dest += &delta,
                };
            }

            let neg_delta = -delta.clone();
            while push_dest != warehouse.robot {
                let next = &push_dest + &neg_delta;
                *aoc_util::try_get_mut(&mut warehouse.items, &push_dest).unwrap() =
                    *aoc_util::try_get(&warehouse.items, &next).unwrap();
                push_dest = next;
            }

            warehouse.robot += delta;
        }
        Command::Up | Command::Down => {
            let delta = command.delta();
            let mut to_explore = vec![&warehouse.robot + &delta];
            let mut to_copy = Vec::new();

            while to_explore.len() > 0 {
                let mut to_explore_next = Vec::new();
                for pt in to_explore.into_iter() {
                    let maybe_lr = match try_get(&warehouse.items, &pt).unwrap() {
                        Item::Wall => return,
                        Item::BoxL => Some((pt.clone(), &pt + &(Command::Right).delta())),
                        Item::BoxR => Some((&pt + &(Command::Left).delta(), pt.clone())),
                        Item::Empty => {
                            to_copy.push(pt);
                            None
                        }
                    };

                    match maybe_lr {
                        Some((l, r)) => {
                            let l_plus_delta = &l + &delta;
                            let r_plus_delta = &r + &delta;
                            match to_explore_next.last() {
                                // The other side of this block was already on this list, dedup.
                                Some(last) if *last == r => (),
                                _ => {
                                    to_explore_next.push(l_plus_delta);
                                    to_explore_next.push(r_plus_delta);
                                    to_copy.push(l);
                                    to_copy.push(r);
                                }
                            };
                        }
                        None => (),
                    };
                }
                to_explore = to_explore_next;
            }

            let robot_dest = &warehouse.robot + &delta;

            let neg_delta = -delta.clone();
            for pt in to_copy.into_iter().rev() {
                if pt.row == robot_dest.row {
                    break;
                }
                *aoc_util::try_get_mut(&mut warehouse.items, &pt).unwrap() =
                    *aoc_util::try_get(&warehouse.items, &(&pt + &neg_delta)).unwrap();
            }

            let maybe_lr = match aoc_util::try_get(&warehouse.items, &robot_dest).unwrap() {
                Item::Wall => panic!(),
                Item::BoxL => Some((robot_dest.clone(), &robot_dest + &(Command::Right).delta())),
                Item::BoxR => Some((&robot_dest + &(Command::Left).delta(), robot_dest.clone())),
                Item::Empty => None,
            };

            match maybe_lr {
                Some((l, r)) => {
                    *aoc_util::try_get_mut(&mut warehouse.items, &l).unwrap() = Item::Empty;
                    *aoc_util::try_get_mut(&mut warehouse.items, &r).unwrap() = Item::Empty;
                }
                None => (),
            }

            warehouse.robot = robot_dest;
        }
    };
}

fn gps(warehouse: &Warehouse) -> usize {
    let mut r = 0;

    for (row, line) in warehouse.items.iter().enumerate() {
        for (col, item) in line.iter().enumerate() {
            if *item == Item::BoxL {
                r += 100 * row + col;
            }
        }
    }

    return r;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let (mut warehouse, commands) = parse_input(fs::read_to_string(&args[1]).unwrap());
    println!("{warehouse}");

    for command in commands {
        exec_command(&mut warehouse, command);
        println!("{warehouse}");
    }

    let g = gps(&warehouse);
    println!("{g}");
}
