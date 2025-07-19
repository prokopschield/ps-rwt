use parking_lot::RwLock;

use crate::inner::RWTi;

impl<R, W> RWTi<R, W> {
    pub const fn new(readonly: R, writable: W) -> Self {
        Self {
            readonly,
            writable: RwLock::new(writable),
        }
    }
}
