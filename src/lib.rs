use std::sync::Arc;

use inner::RWTi;

mod implementations;
mod inner;
mod methods;

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RWT<R, W> {
    inner: Arc<RWTi<R, W>>,
}
