use parking_lot::{lock_api::RwLockReadGuard, RawRwLock};

use crate::RWT;

impl<R, W> RWT<R, W> {
    pub fn read(&self) -> RwLockReadGuard<RawRwLock, W> {
        self.writable.read()
    }
}
