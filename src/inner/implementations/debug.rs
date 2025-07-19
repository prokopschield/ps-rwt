use std::fmt::Debug;

use crate::inner::RWTi;

impl<R, W> Debug for RWTi<R, W>
where
    R: Debug,
    W: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RWT")
            .field(&self.readonly)
            .field(&self.writable)
            .finish()
    }
}
