use crate::RWT;

impl<R, W> Clone for RWT<R, W> {
    fn clone(&self) -> Self {
        Self {
            readonly: self.readonly.clone(),
            writable: self.writable.clone(),
        }
    }
}
