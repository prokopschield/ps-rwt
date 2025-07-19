use crate::RWT;

impl<R, W> Default for RWT<R, W>
where
    R: Default,
    W: Default,
{
    fn default() -> Self {
        Self::new(Default::default(), Default::default())
    }
}
