use crate::RWT;

impl<R, W> Eq for RWT<R, W>
where
    R: Eq,
    W: Eq,
{
}
