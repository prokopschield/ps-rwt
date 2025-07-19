use parking_lot::RwLockWriteGuard;

use crate::RWT;

impl<R, W> RWT<R, W> {
    #[must_use]
    pub fn try_write(&self) -> Option<RwLockWriteGuard<W>> {
        self.writable.try_write()
    }
}
