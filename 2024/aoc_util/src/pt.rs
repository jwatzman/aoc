use std::ops::{Add, AddAssign, Mul, Neg, Rem, RemAssign};

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

impl<T> Rem for Pt<T>
where
    T: num_traits::Euclid,
{
    type Output = Pt<T>;

    fn rem(self, rhs: Self) -> Self::Output {
        Pt {
            row: self.row.rem_euclid(&rhs.row),
            col: self.col.rem_euclid(&rhs.col),
        }
    }
}

impl<T> RemAssign for Pt<T>
where
    T: num_traits::Euclid,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.row = self.row.rem_euclid(&rhs.row);
        self.col = self.row.rem_euclid(&rhs.col);
    }
}
