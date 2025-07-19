use crate::inner::RWTi;

impl<R, W> Eq for RWTi<R, W>
where
    R: Eq,
    W: Eq,
{
}
