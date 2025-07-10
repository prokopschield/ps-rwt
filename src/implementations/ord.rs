use std::cmp::Ordering::{Equal, Greater, Less};

use crate::RWT;

impl<R, W> Ord for RWT<R, W>
where
    R: Ord,
    W: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.readonly.cmp(&other.readonly) {
            Less => Less,
            Greater => Greater,
            Equal => self.writable.read().cmp(&*other.writable.read()),
        }
    }
}
