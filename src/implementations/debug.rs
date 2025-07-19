use std::fmt::Debug;

use crate::RWT;

impl<R, W> Debug for RWT<R, W>
where
    R: Debug,
    W: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.inner, f)
    }
}
