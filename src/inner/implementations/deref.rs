use std::ops::Deref;

use crate::inner::RWTi;

impl<R, W> Deref for RWTi<R, W> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.readonly
    }
}
