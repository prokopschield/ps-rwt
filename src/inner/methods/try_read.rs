use parking_lot::RwLockReadGuard;

use crate::RWT;

impl<R, W> RWT<R, W> {
    #[must_use]
    pub fn try_read(&self) -> Option<RwLockReadGuard<W>> {
        self.writable.try_read()
    }
}
