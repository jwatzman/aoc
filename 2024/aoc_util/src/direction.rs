use crate::pt::Pt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    pub fn rot_left(&self) -> Self {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn rot_right(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn delta<T>(&self) -> Pt<T>
    where
        T: num_traits::Signed,
    {
        let zero = T::zero();
        let one = T::one();
        match *self {
            Direction::Up => Pt {
                row: -one,
                col: zero,
            },
            Direction::Down => Pt {
                row: one,
                col: zero,
            },
            Direction::Left => Pt {
                row: zero,
                col: -one,
            },
            Direction::Right => Pt {
                row: zero,
                col: one,
            },
        }
    }
}
