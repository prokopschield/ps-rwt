use crate::RWT;

impl<R, W> PartialEq for RWT<R, W>
where
    R: PartialEq,
    W: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.readonly == other.readonly && *self.writable.read() == *other.writable.read()
    }
}
