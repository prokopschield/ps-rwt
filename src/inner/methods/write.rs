use parking_lot::{
    lock_api::RwLockWriteGuard, RawRwLock,
};

use crate::RWT;

impl<R, W> RWT<R, W> {
    pub fn write(&self) -> RwLockWriteGuard<RawRwLock, W> {
        self.writable.write()
    }
}
