use std::ops::{Add, AddAssign, Mul, Neg, Rem, RemAssign, Sub};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Pt<T> {
    pub row: T,
    pub col: T,
}

impl<T> Pt<T>
where
    T: Neg<Output = T> + Copy,
{
    pub fn rot_left(&self) -> Self {
        return Pt {
            row: -self.col,
            col: self.row,
        };
    }

    pub fn rot_right(&self) -> Self {
        return Pt {
            row: self.col,
            col: -self.row,
        };
    }
}

impl<T> Add for Pt<T>
where
    T: Add<Output = T>,
{
    type Output = Pt<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Pt {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl<T> Add for &Pt<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Pt<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Pt {
            row: self.row + rhs.row,
            col: self.col + rhs.col,
        }
    }
}

impl<T> AddAssign for Pt<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.row += rhs.row;
        self.col += rhs.col;
    }
}

impl<T> Mul<T> for Pt<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Pt<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Pt {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

impl<T> Mul<T> for &Pt<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Pt<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Pt {
            row: self.row * rhs,
            col: self.col * rhs,
        }
    }
}

fn rem_euclid<T>(n: T, rhs: T) -> T
where
    T: Rem<Output = T> + Add<Output = T> + Sub<Output = T> + PartialOrd + Copy,
{
    let r = n % rhs;
    let z = r - r; // Eww. Not even correct for floats!
    if r >= z {
        return r;
    } else if rhs >= z {
        return r + rhs;
    } else {
        return r - rhs;
    }
}

impl<T> Rem for Pt<T>
where
    T: Rem<Output = T> + Add<Output = T> + Sub<Output = T> + PartialOrd + Copy,
{
    type Output = Pt<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Pt {
            row: rem_euclid(self.row, rhs.row),
            col: rem_euclid(self.col, rhs.col),
        }
    }
}

impl<T> RemAssign for Pt<T>
where
    T: Rem<Output = T> + Add<Output = T> + Sub<Output = T> + PartialOrd + Copy,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.row = rem_euclid(self.row, rhs.row);
        self.col = rem_euclid(self.col, rhs.col);
    }
}
