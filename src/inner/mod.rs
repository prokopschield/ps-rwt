mod implementations;
mod methods;

use parking_lot::RwLock;

#[derive(Default)]
pub struct RWTi<R, W> {
    readonly: R,
    writable: RwLock<W>,
}
