use std::sync::Arc;

use parking_lot::RwLock;

use crate::RWT;

impl<R, W> RWT<R, W> {
    pub fn new(readonly: R, writable: W) -> Self {
        Self::from_parts(Arc::new(readonly), Arc::new(RwLock::new(writable)))
    }
}
