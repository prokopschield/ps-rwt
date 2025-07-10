use parking_lot::{ArcRwLockReadGuard, RawRwLock};

use crate::RWT;

impl<R, W> RWT<R, W> {
    pub fn read(&self) -> ArcRwLockReadGuard<RawRwLock, W> {
        self.writable.read_arc()
    }
}
