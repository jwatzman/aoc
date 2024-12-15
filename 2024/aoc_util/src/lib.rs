mod pt;
pub use pt::Pt;

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
