use parking_lot::{ArcRwLockWriteGuard, RawRwLock};

use crate::RWT;

impl<R, W> RWT<R, W> {
    pub fn write(&self) -> ArcRwLockWriteGuard<RawRwLock, W> {
        self.writable.write_arc()
    }
}
