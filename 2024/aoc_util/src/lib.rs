pub mod pt;
pub use pt::Pt;

pub mod direction;
pub use direction::Direction;

pub fn try_get<'a, T, U>(vv: &'a Vec<Vec<T>>, pt: &Pt<U>) -> Option<&'a T>
where
    T: Copy,
    U: Copy,
    usize: TryFrom<U>,
{
    let row = Result::ok(usize::try_from(pt.row))?;
    let col = Result::ok(usize::try_from(pt.col))?;
    return Some(vv.get(row)?.get(col)?);
}

pub fn try_get_mut<'a, T, U>(vv: &'a mut Vec<Vec<T>>, pt: &Pt<U>) -> Option<&'a mut T>
where
    T: Copy,
    U: Copy,
    usize: TryFrom<U>,
{
    let row = Result::ok(usize::try_from(pt.row))?;
    let col = Result::ok(usize::try_from(pt.col))?;
    return Some(vv.get_mut(row)?.get_mut(col)?);
}

// Adapted from
// https://gist.github.com/victor-iyi/8a84185c1d52419b0d4915a648d5e3e1
pub fn gcd<T>(mut n: T, mut m: T) -> T
where
    T: num_traits::Num + PartialOrd + Copy,
{
    if n.is_zero() {
        return m;
    }

    if m.is_zero() {
        return n;
    }

    // No way to do abs for unsigned! Ugly hack:
    let z = T::zero();
    if n < z {
        n = n - n - n;
    }
    if m < z {
        m = m - m - m;
    }

    while !m.is_zero() {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m = m % n;
    }

    return n;
}

#[cfg(test)]
mod tests {
    #[test]
    fn gcd() {
        assert_eq!(super::gcd(6, 9), 3);
        assert_eq!(super::gcd(6, -9), 3);
        assert_eq!(super::gcd(-6, 9), 3);
        assert_eq!(super::gcd(-6, -9), 3);

        assert_eq!(super::gcd(6_usize, 9_usize), 3);

        assert_eq!(super::gcd(7, 0), 7);
        assert_eq!(super::gcd(0, 7), 7);
    }
}
