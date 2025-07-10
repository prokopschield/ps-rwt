use std::sync::Arc;

use parking_lot::RwLock;

use crate::RWT;

impl<R, W> RWT<R, W> {
    pub const fn from_parts(readonly: Arc<R>, writable: Arc<RwLock<W>>) -> Self {
        Self { readonly, writable }
    }
}
