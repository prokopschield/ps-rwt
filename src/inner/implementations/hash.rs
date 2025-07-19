use std::hash::Hash;

use crate::inner::RWTi;

impl<R, W> Hash for RWTi<R, W>
where
    R: Hash,
    W: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.readonly.hash(state);
        self.writable.read().hash(state);
    }
}
