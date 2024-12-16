use std::ops::{Add, AddAssign, Mul, Neg, Rem, RemAssign};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Pt<T> {
    pub row: T,
    pub col: T,
}

impl<T> Pt<T> {
    pub fn try_from<U>(row: U, col: U) -> Result<Self, <T as TryFrom<U>>::Error>
    where
        T: TryFrom<U>,
    {
        let row = T::try_from(row)?;
        let col = T::try_from(col)?;
        Ok(Pt { row, col })
    }
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

impl<T> Neg for Pt<T>
where
    T: Neg<Output = T>,
{
    type Output = Pt<T>;

    fn neg(self) -> Self::Output {
        Pt {
            row: -self.row,
            col: -self.col,
        }
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

impl<T> Add<Pt<T>> for &Pt<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Pt<T>;
    fn add(self, rhs: Pt<T>) -> Self::Output {
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

impl<T> AddAssign<&Pt<T>> for Pt<T>
where
    T: AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: &Self) {
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
        self.col = self.col.rem_euclid(&rhs.col);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_mul() {
        let p1 = Pt { row: 3, col: 4 };
        let p2 = Pt { row: 5, col: 6 };

        let p = p1.clone() + p2.clone();
        assert_eq!(p.row, 8);
        assert_eq!(p.col, 10);

        let mut p = p1.clone();
        p += p2.clone();
        assert_eq!(p.row, 8);
        assert_eq!(p.col, 10);

        let p = p1.clone() * 3;
        assert_eq!(p.row, 9);
        assert_eq!(p.col, 12);
    }

    #[test]
    fn rem() {
        let p1 = Pt { row: 6, col: -10 };

        let p = p1.clone() % Pt { row: 5, col: 3 };
        assert_eq!(p.row, 1);
        assert_eq!(p.col, 2);

        let p = p1.clone() % Pt { row: -5, col: -3 };
        assert_eq!(p.row, 1);
        assert_eq!(p.col, 2);
    }

    #[test]
    fn try_from() -> Result<(), Box<dyn std::error::Error>> {
        let p = Pt::<i32>::try_from(3_usize, 4_usize)?;
        assert_eq!(p.row, 3);
        assert_eq!(p.col, 4);

        let p = Pt::<i32>::try_from(usize::MAX, 0);
        assert!(p.is_err());

        Ok(())
    }
}
