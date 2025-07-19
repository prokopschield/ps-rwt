use crate::RWT;

impl<R, W> Clone for RWT<R, W> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
