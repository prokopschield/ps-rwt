use std::sync::Arc;

use parking_lot::RwLock;

mod implementations;
mod methods;

pub struct RWT<R, W> {
    readonly: Arc<R>,
    writable: Arc<RwLock<W>>,
}
