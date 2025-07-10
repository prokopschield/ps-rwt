use parking_lot::{ArcRwLockReadGuard, RawRwLock};

use crate::RWT;

impl<R, W> RWT<R, W> {
    #[must_use]
    pub fn try_read(&self) -> Option<ArcRwLockReadGuard<RawRwLock, W>> {
        self.writable.try_read_arc()
    }
}
