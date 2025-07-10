use parking_lot::{ArcRwLockWriteGuard, RawRwLock};

use crate::RWT;

impl<R, W> RWT<R, W> {
    #[must_use]
    pub fn try_write(&self) -> Option<ArcRwLockWriteGuard<RawRwLock, W>> {
        self.writable.try_write_arc()
    }
}
