use std::env;
use std::fmt::Display;
use std::fs;

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

/*
fn exec_command(warehouse: &mut Warehouse, command: Command) {
    let delta = command.delta();
    let robot_dest = &warehouse.robot + &delta;
    let mut push_dest = robot_dest.clone();

    loop {
        match aoc_util::try_get(&warehouse.items, &push_dest).unwrap() {
            Item::Wall => return,
            Item::Empty => break,
            Item::Box => push_dest += &delta,
        }
    }

    *aoc_util::try_get_mut(&mut warehouse.items, &push_dest).unwrap() = Item::Box;
    *aoc_util::try_get_mut(&mut warehouse.items, &robot_dest).unwrap() = Item::Empty;
    warehouse.robot = robot_dest;
}
*/

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

    /*
    for command in commands {
        exec_command(&mut warehouse, command);
        //println!("{warehouse}");
    }

    let g = gps(&warehouse);
    println!("{g}");
    */
}
