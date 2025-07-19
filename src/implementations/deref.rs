use std::ops::Deref;

use crate::{inner::RWTi, RWT};

impl<R, W> Deref for RWT<R, W> {
    type Target = RWTi<R, W>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
