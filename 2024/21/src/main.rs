use std::env;
use std::fmt::Display;
use std::fs;
use std::iter;

type RC = i8;

trait Button: Copy + Eq {
    fn row(&self) -> RC;
    fn col(&self) -> RC;

    const HOLE: (RC, RC);
    const START: Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NumericButton {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Activate,
}

impl Button for NumericButton {
    fn row(&self) -> RC {
        match *self {
            Self::Seven | Self::Eight | Self::Nine => 0,
            Self::Four | Self::Five | Self::Six => 1,
            Self::One | Self::Two | Self::Three => 2,
            Self::Zero | Self::Activate => 3,
        }
    }

    fn col(&self) -> RC {
        match *self {
            Self::Seven | Self::Four | Self::One => 0,
            Self::Eight | Self::Five | Self::Two | Self::Zero => 1,
            Self::Nine | Self::Six | Self::Three | Self::Activate => 2,
        }
    }

    const HOLE: (RC, RC) = (3, 0);
    const START: Self = NumericButton::Activate;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirectionalButton {
    Up,
    Down,
    Left,
    Right,
    Activate,
}

impl Display for DirectionalButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match *self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
            Self::Activate => 'A',
        };
        write!(f, "{c}")
    }
}

impl Button for DirectionalButton {
    fn row(&self) -> RC {
        match *self {
            Self::Up | Self::Activate => 0,
            Self::Left | Self::Down | Self::Right => 1,
        }
    }

    fn col(&self) -> RC {
        match *self {
            Self::Left => 0,
            Self::Up | Self::Down => 1,
            Self::Activate | Self::Right => 2,
        }
    }

    const HOLE: (RC, RC) = (0, 0);
    const START: Self = DirectionalButton::Activate;
}

fn parse_input(contents: String) -> Vec<(Vec<NumericButton>, usize)> {
    let mut codes = Vec::new();

    for line in contents.split('\n') {
        if line.is_empty() {
            continue;
        }

        let mut code = Vec::new();
        for c in line.chars() {
            code.push(match c {
                '0' => NumericButton::Zero,
                '1' => NumericButton::One,
                '2' => NumericButton::Two,
                '3' => NumericButton::Three,
                '4' => NumericButton::Four,
                '5' => NumericButton::Five,
                '6' => NumericButton::Six,
                '7' => NumericButton::Seven,
                '8' => NumericButton::Eight,
                '9' => NumericButton::Nine,
                'A' => NumericButton::Activate,
                _ => panic!(),
            })
        }

        codes.push((code, line[0..3].parse().unwrap()));
    }

    return codes;
}

fn expand_button<T: Button>(cur_loc: T, next_loc: T) -> Vec<DirectionalButton> {
    let mut out = Vec::new();

    let row_diff = next_loc.row() - cur_loc.row();
    let col_diff = next_loc.col() - cur_loc.col();

    let row_presses = iter::repeat(if row_diff < 0 {
        DirectionalButton::Up
    } else {
        DirectionalButton::Down
    })
    .take(row_diff.abs().try_into().unwrap());

    let col_presses = iter::repeat(if col_diff < 0 {
        DirectionalButton::Left
    } else {
        DirectionalButton::Right
    })
    .take(col_diff.abs().try_into().unwrap());

    // If we are forced to avoid the hole, do that. If we have a choice,
    // some options are actually better than others -- see
    // https://www.reddit.com/r/adventofcode/comments/1hj7f89/comment/m34erhg/
    if cur_loc.col() == T::HOLE.1 && next_loc.row() == T::HOLE.0 {
        out.extend(col_presses);
        out.extend(row_presses);
    } else if cur_loc.row() == T::HOLE.0 && next_loc.col() == T::HOLE.1 {
        out.extend(row_presses);
        out.extend(col_presses);
    } else if col_diff < 0 {
        out.extend(col_presses);
        out.extend(row_presses);
    } else {
        out.extend(row_presses);
        out.extend(col_presses);
    }

    out.push(DirectionalButton::Activate);
    return out;
}

fn count_expanded_buttons<T: Button>(code: &Vec<T>, depth: u8) -> usize {
    if depth == 0 {
        return code.len();
    }

    let mut cur_loc = T::START;
    let mut r = 0;

    for next_loc in code {
        let next_code = expand_button(cur_loc, *next_loc);
        r += count_expanded_buttons(&next_code, depth - 1);
        cur_loc = *next_loc;
    }

    if cur_loc != T::START {
        panic!();
    }

    return r;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let codes = parse_input(fs::read_to_string(&args[1]).unwrap());

    let mut r = 0;
    for code in &codes {
        let cnt = count_expanded_buttons(&code.0, 3);
        r += cnt * code.1;
    }

    println!("{r}");
}
