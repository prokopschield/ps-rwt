use std::{ops::Deref, sync::Arc};

use crate::RWT;

impl<R, W> Deref for RWT<R, W> {
    type Target = Arc<R>;

    fn deref(&self) -> &Self::Target {
        &self.readonly
    }
}
