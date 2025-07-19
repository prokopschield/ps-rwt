use std::cmp::Ordering::{Equal, Greater, Less};

use crate::inner::RWTi;

impl<R, W> PartialOrd for RWTi<R, W>
where
    R: PartialOrd,
    W: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.readonly.partial_cmp(&other.readonly) {
            None => None,
            Some(Less) => Some(Less),
            Some(Greater) => Some(Greater),
            Some(Equal) => self.writable.read().partial_cmp(&*other.writable.read()),
        }
    }
}
