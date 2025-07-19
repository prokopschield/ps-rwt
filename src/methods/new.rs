use std::sync::Arc;

use crate::{inner::RWTi, RWT};

impl<R, W> RWT<R, W> {
    pub fn new(readonly: R, writable: W) -> Self {
        Self {
            inner: Arc::new(RWTi::new(readonly, writable)),
        }
    }
}
